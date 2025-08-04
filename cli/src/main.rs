use codegen::Codegen;
use lex::Lexer;
use parse::Parser;
use std::{
    error::Error,
    fs::OpenOptions,
    io::{Write, stderr, stdout},
    path::PathBuf,
    process::Command,
};
use tacky::lower;
use tracing::{error, info};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

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

/// A validated wrapper over args where the file is known to be valid and has already been read
struct File {
    args: Args,
    contents: String,
}

impl TryFrom<Args> for File {
    type Error = std::io::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let contents = std::fs::read_to_string(&args.file)?;
        Ok(File { args, contents })
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
    println!("  --codegen: Runs up to codegen but doesn't emit any file");
    println!("  -S: Emits naked assembly file");
    println!("  --full: Runs the whole pipeline and outputs final executable");
}

fn lex(file: File) {
    let output = Lexer::lex(&file.contents);

    println!("{output}");
}

fn parse(file: File) {
    let output = Lexer::lex(&file.contents);
    let mut parser = Parser::from_tokens(&output);
    parser.parse();
    println!("{}", parser.nodes());
}

fn tacky(file: File) {
    let output = Lexer::lex(&file.contents);
    let mut parser = Parser::from_tokens(&output);
    parser.parse();
    let tacky = lower(parser.nodes());
    println!("{}", tacky);
}

fn codegen(file: File) {
    let output = Lexer::lex(&file.contents);
    let mut parser = Parser::from_tokens(&output);
    parser.parse();
    let tacky = lower(parser.nodes());
    let mut codegen = Codegen::new(&tacky);
    codegen.emit();
    println!("{}", codegen.output());
}

fn naked_assembly(file: File) {
    let output = Lexer::lex(&file.contents);
    let mut parser = Parser::from_tokens(&output);
    parser.parse();
    let tacky = lower(parser.nodes());
    let mut codegen = Codegen::new(&tacky);
    codegen.emit();
    let mut output_file = file.args.file.clone();
    output_file.set_extension("s");
    std::fs::write(output_file, codegen.output());
}

fn full(file: File) {
    let output = Lexer::lex(&file.contents);
    let mut parser = Parser::from_tokens(&output);
    parser.parse();
    let tacky = lower(parser.nodes());
    let mut codegen = Codegen::new(&tacky);
    codegen.emit();
    let mut assembly_file = file.args.file.clone();
    assembly_file.set_extension("s");
    std::fs::write(&assembly_file, codegen.output());
    let mut executable_file = assembly_file.clone();
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
        Ok(args) => match File::try_from(args) {
            Ok(file) => match file.args.mode {
                CompilationMode::Lex => lex(file),
                CompilationMode::Parse => parse(file),
                CompilationMode::Tacky => codegen(file),
                CompilationMode::Codegen => codegen(file),
                CompilationMode::NakedAssembly => naked_assembly(file),
                CompilationMode::Full => full(file),
            },
            Err(e) => {
                error!("Error reading file {e:?}");
                eprintln!("Error reading file {e:?}");
            }
        },
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
