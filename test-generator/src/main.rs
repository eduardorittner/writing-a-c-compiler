use std::path::PathBuf;

mod test;

use test::{CompilationMode, TestSuite};

struct Options {
    chapter: u8,
    sources: PathBuf,
    output: PathBuf,
    stage: CompilationMode,
    test_invalid: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            chapter: 1,
            sources: PathBuf::from("../examples"),
            output: PathBuf::from("../tests/tests"),
            stage: CompilationMode::Lex,
            test_invalid: false,
        }
    }
}

fn parse_args() -> Options {
    let mut options = Options::default();
    let mut args: Vec<_> = std::env::args().collect();

    args.remove(0);

    loop {
        if args.is_empty() {
            break;
        }

        let arg = args.remove(0);
        match arg.as_str() {
            "--test-invalid" => options.test_invalid = true,
            "--chapter" => {
                let arg = args.remove(0);
                if let Ok(chapter) = arg.parse() {
                    options.chapter = chapter;
                } else {
                    panic!("Expected a chapter after '--chapter', got: {arg}");
                }
            }
            "--stage" => {
                let arg = args.remove(0);
                match arg.as_str() {
                    "lex" => options.stage = CompilationMode::Lex,
                    "parse" => options.stage = CompilationMode::Parse,
                    "tacky" => options.stage = CompilationMode::Tacky,
                    "codegen" => options.stage = CompilationMode::Codegen,
                    "run" => todo!(),
                    _ => panic!("Unknown option: {arg}"),
                }
            }
            _ => panic!("Unknown option: {arg}"),
        }
    }

    options
}

fn main() {
    let options = parse_args();

    let chapters = options
        .sources
        .read_dir()
        .unwrap()
        .map(|p| p.unwrap().path());

    chapters
        .map(|p| TestSuite::from(p))
        .filter(|t| t.chapter <= options.chapter)
        .for_each(|t| t.generate_tests(&options.output, options.test_invalid, options.stage));
}
