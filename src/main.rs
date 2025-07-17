use lex::{Lexer, TokenType};
use std::{error::Error, path::PathBuf};

/// Cli arguments
///
/// rustcc <path> --[lex|parse|codegen]
struct Args {
    file: PathBuf,
    mode: CompilationMode,
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

    // First argument is the executable name
    args.remove(0);

    if args.len() == 0 {
        // TODO No file argument
        return Err(CliError);
    }

    let file: PathBuf = args.remove(0).into();

    if !file.exists() {
        // TODO No such file exists
        return Err(CliError);
    }

    if args.len() == 0 {
        return Ok(Args {
            file,
            mode: CompilationMode::default(),
        });
    }

    if args.len() > 1 {
        // TODO Too many arguments
        return Err(CliError);
    }

    // TODO invalid flag arg
    let arg = args.remove(0).into_string().map_err(|_| CliError)?;

    let mode = match arg.as_str() {
        "--lex" => CompilationMode::Lex,
        "--parse" => CompilationMode::Parse,
        "--codegen" => CompilationMode::Codegen,
        "-S" => CompilationMode::NakedAssembly,
        // TODO invalid flag arg
        _ => return Err(CliError),
    };

    Ok(Args { file, mode })
}

fn usage_help() {
    todo!()
}

fn lex(file: File) {
    let output = Lexer::lex(&file.contents);

    // TODO implement display for tokenized output?
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
