use lex::Lexer;
use std::{error::Error, path::PathBuf};

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
    /// Stop after codegen, doesn't emit assembly file
    Codegen,
    /// Stop after codegen but emits the assembly file
    NakedAssembly,
    /// Emits the final executable
    #[default]
    Full,
}

// TODO add variants
// 1. Missing file path
// 2. Invalid file path
// 3. Invalid flags
#[derive(Debug, PartialEq)]
struct CliError;

impl std::fmt::Display for CliError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for CliError {}

fn parse_args(mut args: Vec<String>) -> Result<Args, CliError> {
    let mut constructed_args = Args::default();

    // First argument is the executable name
    args.remove(0);

    if args.is_empty() {
        // TODO No file argument
        return Err(CliError);
    }

    while let Some(arg) = args.pop() {
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
    if let Ok(args) = parse_args(
        std::env::args_os()
            .map(|s| s.into_string().unwrap())
            .collect(),
    ) {
        match File::try_from(args) {
            Ok(file) => match file.args.mode {
                CompilationMode::Lex => lex(file),
                CompilationMode::Parse => todo!(),
                CompilationMode::Codegen => todo!(),
                CompilationMode::NakedAssembly => todo!(),
                CompilationMode::Full => todo!(),
            },
            Err(e) => {
                eprintln!("Error reading file {e:?}");
            }
        }
    } else {
        usage_help()
    };
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

        assert_eq!(args, Err(CliError));
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

        assert_eq!(args, Err(CliError));
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
