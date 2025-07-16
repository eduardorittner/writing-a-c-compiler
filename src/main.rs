<<<<<<< Updated upstream
use lex::TokenType;
use std::{error::Error, path::PathBuf};

/// Cli arguments
///
/// rustcc <path> --[lex|parse|codegen]
struct Args {
    file: PathBuf,
    mode: CompilationMode,
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

fn main() {
    println!("Hello, world!");
||||||| Stash base
fn main() {
    println!("Hello, world!");
=======
#![feature(variant_count)]

type LitId = u16;

mod ttype;

use crate::ttype::TokenInfo;
use crate::ttype::TokenType;

struct Token {
    id: [u8; 3],    // u24
    tag: TokenType, // tag + error
}

fn main() {
    let i = TokenInfo::new(TokenType::Semicolon, false);
    println!("{}", ttype::ERROR);
}

#[cfg(test)]
mod tests {
    use crate::Token;

    #[test]
    fn token_size() {
        assert_eq!(4, size_of::<Token>());
    }
>>>>>>> Stashed changes
}
