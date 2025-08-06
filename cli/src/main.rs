use ast::Tree;
use codegen::Codegen;
use lex::{LexError, Lexer, TokenizedOutput};
use parse::Parser;
use std::{
    error::Error,
    fs::OpenOptions,
    io::{Write, stderr, stdout},
    path::{Path, PathBuf},
    process::Command,
};
use tracing::{error, info};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use x86::{X86, lower};

/// Cli arguments
///
/// rustcc <path> --[lex|parse|codegen]
#[derive(Debug, PartialEq)]
struct Args {
    file: PathBuf,
    mode: CompilationMode,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            file: PathBuf::default(),
            mode: CompilationMode::Full,
        }
    }
}

/// Compilation mode
/// Default is Full, which generates the executable
#[derive(Debug, PartialEq, Default)]
enum CompilationMode {
    /// Stop after lexing
    Lex,
    /// Stop after parsing
    Parse,
    /// Stop after generating tacky IR
    Tacky,
    /// Stop after codegen, doesn't emit assembly file
    Codegen,
    /// Emits the generated assembly
    NakedAssembly,
    /// Emits the final linked executable
    #[default]
    Full,
}

// TODO add variants
// 1. Missing file path
// 2. Invalid file path
// 3. Invalid flags
#[derive(Debug, PartialEq)]
enum CliError {
    NoFileArg,
    NoSuchFile,
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::NoFileArg => write!(f, "No file was provided"),
            CliError::NoSuchFile => write!(f, "File doesn't exist"),
        }
    }
}

impl Error for CliError {}

fn parse_args(mut args: Vec<String>) -> Result<Args, CliError> {
    let mut constructed_args = Args::default();

    // First argument is the executable name
    args.remove(0);

    if args.is_empty() {
        // TODO No file argument
        return Err(CliError::NoFileArg);
    }

    while let Some(arg) = args.pop() {
        match arg.as_str() {
            "--lex" => constructed_args.mode = CompilationMode::Lex,
            "--parse" => constructed_args.mode = CompilationMode::Parse,
            "--tacky" => constructed_args.mode = CompilationMode::Tacky,
            "--codegen" => constructed_args.mode = CompilationMode::Codegen,
            "-S" => constructed_args.mode = CompilationMode::NakedAssembly,
            "--full" => constructed_args.mode = CompilationMode::Full,
            file => {
                let path: PathBuf = file.into();
                constructed_args.file = path;
            }
        };
    }

    if constructed_args.file == PathBuf::default() {
        return Err(CliError::NoFileArg);
    }

    if !constructed_args.file.exists() {
        error!("No such file: {:?}", constructed_args.file);
        eprintln!("{:?}", constructed_args.file);
        // TODO No such file exists
        return Err(CliError::NoSuchFile);
    }

    error!("Parsed args: {constructed_args:?}");

    Ok(constructed_args)
}

fn usage_help() {
    println!("Usage:");
    println!("ccompiler [file] [options]");
    println!();
    println!("Options:");
    println!("  --lex: Only runs the lexer");
    println!("  --parse: Only runs the parser");
    println!("  --tacky: Runs up to tacky lowering");
    println!("  --codegen: Runs up to codegen but doesn't emit any file");
    println!("  -S: Emits naked assembly file");
    println!("  --full: Runs the whole pipeline and outputs final executable");
}

pub fn lex<'src>(src: &'src str) -> Result<TokenizedOutput<'src>, Box<dyn Error>> {
    Ok(Lexer::lex(src)?)
}

fn parse<'src>(src: &'src str) -> Result<Tree<'src>, Box<dyn Error>> {
    let tokens = lex(src)?;
    let mut parser = Parser::from_tokens(tokens);
    parser.parse();
    Ok(parser.nodes.clone())
}

fn tacky<'src>(src: &'src str) -> Result<X86, Box<dyn Error>> {
    let ast = parse(src)?;
    Ok(lower(&ast))
}

fn codegen(src: &str) -> Result<String, Box<dyn Error>> {
    let tacky = tacky(src)?;
    let mut codegen = Codegen::new(&tacky);
    codegen.emit();
    Ok(codegen.output().to_string())
}

fn naked_assembly(src: &str, output_file: &Path) -> Result<(), Box<dyn Error>> {
    let assembly = codegen(src)?;
    std::fs::write(output_file, assembly)?;
    Ok(())
}

fn full(src: &str, input_file: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let mut assembly_file = PathBuf::from(input_file);
    assembly_file.set_extension("s");

    naked_assembly(src, &assembly_file)?;
    let mut executable_file = PathBuf::from(input_file);
    executable_file.set_extension("");

    let mut linker = Command::new("cc");
    linker.arg(&assembly_file).arg("-o").arg(&executable_file);

    info!(
        "Running command: {:?} {:?}",
        linker.get_program(),
        linker.get_args()
    );

    let linker_output = linker.output();

    info!("Deleting intermediate assembly file: {assembly_file:?}");

    let _ = std::fs::remove_file(&assembly_file);

    match linker_output {
        Ok(ok) => {
            info!("Executable generated");
            stdout().write_all(&ok.stdout);
            stderr().write_all(&ok.stderr);
        }
        Err(e) => {
            error!("Got linker error: {e:?}");
            panic!("Linker error: {e:?}");
        }
    };
    Ok(executable_file)
}

fn main() {
    // Create a log file, overriding any pre-existing ones
    let log_file = OpenOptions::new()
        .append(false)
        .write(true)
        .truncate(true)
        .create(true)
        .open("ccompiler.log")
        .unwrap();

    // Log output configuration
    tracing_subscriber::registry()
        // Log to stdout
        .with(fmt::layer().with_span_events(FmtSpan::CLOSE))
        // Log to log file
        .with(
            fmt::layer()
                .with_writer(log_file)
                .with_ansi(false)
                .with_span_events(FmtSpan::CLOSE),
        )
        .init();

    info!("Starting");

    match parse_args(
        std::env::args_os()
            .map(|s| s.into_string().unwrap())
            .collect(),
    ) {
        Ok(args) => {
            let input = std::fs::read_to_string(&args.file).unwrap();

            match args.mode {
                CompilationMode::Lex => match lex(&input) {
                    Ok(tokens) => println!("{}", tokens),
                    Err(e) => panic!("{}", e),
                },
                CompilationMode::Parse => match parse(&input) {
                    Ok(ast) => println!("{}", ast),
                    Err(e) => panic!("{}", e),
                },
                CompilationMode::Tacky => match tacky(&input) {
                    Ok(tacky) => println!("{}", tacky),
                    Err(e) => panic!("{}", e),
                },
                CompilationMode::Codegen => match codegen(&input) {
                    Ok(assembly) => println!("{}", assembly),
                    Err(e) => panic!("{}", e),
                },
                CompilationMode::NakedAssembly => {
                    let mut assembly_file = PathBuf::from(&args.file);
                    assembly_file.set_extension("s");
                    match naked_assembly(&input, &assembly_file) {
                        Ok(()) => println!("Generated {assembly_file:?}"),
                        Err(e) => panic!("{}", e),
                    }
                }
                CompilationMode::Full => match full(&input, &args.file) {
                    Ok(executable) => println!("Generated {executable:?}"),
                    Err(e) => panic!("{}", e),
                },
            }
        }
        Err(e) => {
            error!("Invalid args: {e:?}");
            eprintln!("Invalid args: {e:?}");
            usage_help()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{Args, CliError, CompilationMode, parse_args};

    macro_rules! args [
        ($x:expr) => (
            $x.into_iter().map(|s| s.to_string()).collect()
        );
    ];

    #[test]
    fn no_args() {
        let args = args![vec![""]];
        let args = parse_args(args);

        assert_eq!(args, Err(CliError::NoFileArg));
    }

    #[test]
    fn one_arg() {
        // File needs to exist
        let file = "Cargo.toml";
        let args = args![vec!["", file]];
        let args = parse_args(args);

        assert_eq!(
            args,
            Ok(Args {
                file: PathBuf::from(file),
                mode: CompilationMode::Full
            })
        );
    }

    #[test]
    fn flag_with_no_file() {
        let args = args![vec!["", "--lex"]];
        let args = parse_args(args);

        assert_eq!(args, Err(CliError::NoFileArg));
    }

    #[test]
    fn flag_with_file() {
        let file = "Cargo.toml";
        let args = args![vec!["", file, "--lex"]];
        let args = parse_args(args);

        assert_eq!(
            args,
            Ok(Args {
                file: PathBuf::from(file),
                mode: CompilationMode::Lex
            })
        );
    }

    #[test]
    fn flag_with_file_order_shouldnt_matter() {
        let file = "Cargo.toml";

        assert_eq!(
            parse_args(args![vec!["", file, "--lex"]]),
            parse_args(args![vec!["", "--lex", file]])
        );
    }
}
