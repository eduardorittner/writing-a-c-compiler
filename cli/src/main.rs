use std::{fs::OpenOptions, path::PathBuf};
use tracing::{error, info};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use cli::*;
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
