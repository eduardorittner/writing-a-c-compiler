use std::path::PathBuf;

mod test;

use test::TestSuite;

struct Options {
    chapter: u8,
    clean: bool,
    sources: PathBuf,
    output: PathBuf,
    stages: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            chapter: 1,
            clean: false,
            sources: PathBuf::from("../examples"),
            output: PathBuf::from("../tests/tests"),
            stages: vec!["valid".to_string()],
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
        if let Ok(chapter) = arg.parse::<u8>() {
            options.chapter = chapter
        } else {
            match arg.as_str() {
                "clean" => options.clean = true,
                arg if matches!(arg, "valid" | "invalid_lex" | "invalid_parse") => {
                    options.stages.push(arg.to_string());
                }
                _ => panic!("Unknown option: {arg}"),
            }
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

    let test_suites = chapters
        .map(|p| TestSuite::from(p))
        .filter(|t| t.chapter <= options.chapter)
        .for_each(|t| t.generate_tests(&options.output, &options.stages));
}
