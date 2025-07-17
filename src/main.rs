use lex::{Lexer, TokenType};
use std::{error::Error, path::PathBuf};

/// Cli arguments
///
/// rustcc <path> --[lex|parse|codegen]
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
#[derive(Debug)]
enum CompilationMode {
    /// Stop after lexing
    Lex,
    /// Stop after parsing
    Parse,
    /// Stop after codegen, doesn't emit assembly file
    Codegen,
    /// Stop after codegen but emits the assembly file
    NakedAssembly,
    /// Emits the final executable
    Full,
}

impl Default for CompilationMode {
    fn default() -> Self {
        CompilationMode::Full
    }
}

// TODO add variants
// 1. Missing file path
// 2. Invalid file path
// 3. Invalid flags
#[derive(Debug)]
struct CliError;

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for CliError {}

fn parse_args() -> Result<Args, CliError> {
    let mut args: Vec<_> = std::env::args_os().collect();
    let mut constructed_args = Args::default();

    // First argument is the executable name
    args.remove(0);

    if args.len() == 0 {
        // TODO No file argument
        return Err(CliError);
    }

    while let Some(arg) = args.pop() {
        let arg = arg.into_string().unwrap();
        match arg.as_str() {
            "--lex" => constructed_args.mode = CompilationMode::Lex,
            "--parse" => constructed_args.mode = CompilationMode::Parse,
            "--codegen" => constructed_args.mode = CompilationMode::Codegen,
            "-S" => constructed_args.mode = CompilationMode::NakedAssembly,
            file => {
                let path: PathBuf = file.into();
                constructed_args.file = path;
            }
        };
    }

    if !constructed_args.file.exists() {
        eprintln!("{:?}", constructed_args.file);
        // TODO No such file exists
        return Err(CliError);
    }

    Ok(constructed_args)
}

fn usage_help() {
    todo!()
}

fn lex(file: File) {
    let output = Lexer::lex(&file.contents);

    println!("{output}");
}

fn main() {
    if let Ok(args) = parse_args() {
        match File::try_from(args) {
            Ok(file) => match file.args.mode {
                CompilationMode::Lex => lex(file),
                CompilationMode::Parse => todo!(),
                CompilationMode::Codegen => todo!(),
                CompilationMode::NakedAssembly => todo!(),
                CompilationMode::Full => todo!(),
            },
            Err(e) => {
                eprintln!("Error reading file {:?}", e);
            }
        }
    } else {
        usage_help()
    };
}
