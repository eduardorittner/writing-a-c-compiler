use std::path::{Path, PathBuf};

// TODO: import this from the cli crate
// Can't do this yet because cli is a binary crate. Maybe we should turn it into a library and have
// a `bin` crate which just calls `parse_args` and stuff.
#[derive(Clone, Copy)]
pub enum CompilationMode {
    Lex,
    Parse,
    Tacky,
    Codegen,
    NakedAssembly,
    // TODO do we need this? Since we don't want to actually emit any files
    Full,
}

pub struct Test {
    pub source_file: PathBuf,
}

impl Test {
    pub fn name(&self) -> &str {
        self.source_file.file_stem().unwrap().to_str().unwrap()
    }

    fn generate_test_case(&self, stage: CompilationMode) -> String {
        let mut code = String::new();

        code.push_str("#[test]\n");
        code.push_str(&format!("fn {}() {{\n", self.name()));
        code.push_str(&format!(
            "let source = std::fs::read_to_string({:?}).unwrap();\n",
            self.source_file
        ));

        match stage {
            CompilationMode::Lex => {
                code.push_str("let tokens = Lexer::lex(&source);\n");
                code.push_str("insta::assert_debug_snapshot!(tokens);\n");
            }
            CompilationMode::Parse => {
                code.push_str("let tokens = Lexer::lex(&source);\n");
                code.push_str("let parser = Parser::from_tokens(tokens);\n");
                code.push_str("parser.parse();\n");
                code.push_str("insta::assert_debug_snapshot!(tokens);\n");
            }
            CompilationMode::Tacky | CompilationMode::Codegen | CompilationMode::NakedAssembly => {
                todo!()
            }
            CompilationMode::Full => todo!(),
        };

        code.push_str("}\n");

        code
    }
}

/// Represents a leaf directory from the official tests
pub struct TestSuite {
    pub chapter: u8,
    pub valid: Option<Vec<Test>>,
    pub invalid_lex: Option<Vec<Test>>,
    pub invalid_parse: Option<Vec<Test>>,
}

impl TestSuite {
    pub fn generate_tests(&self, output_dir: &Path, test_invalid: bool, stage: CompilationMode) {
        println!("generating tests");
        self.generate_valid_tests(output_dir, stage);
        if test_invalid {
            self.generate_invalid_lex_tests(output_dir, stage);
            self.generate_invalid_parse_tests(output_dir, stage);
        }
    }

    pub fn generate_valid_tests(&self, output_dir: &Path, stage: CompilationMode) {
        let imports = self.generate_imports();
        if let Some(valid) = &self.valid {
            let test_cases = valid
                .iter()
                .map(|test| test.generate_test_case(stage))
                .reduce(|mut acc, s| {
                    acc.push_str(&s);
                    acc
                })
                .unwrap();

            let file_contents = format!("{}\n{}", imports, test_cases);

            let output_file = format!("chapter_{}.rs", self.chapter);
            let mut output_path = output_dir.to_owned();
            output_path.push(output_file);

            if let Err(e) = std::fs::write(&output_path, file_contents) {
                panic!("Unable to write to file {output_path:?}: {e}");
            }
        }
    }

    pub fn generate_invalid_lex_tests(&self, output_dir: &Path, stage: CompilationMode) {
        let imports = self.generate_imports();
        if let Some(invalid_lex) = &self.invalid_lex {
            let test_cases = invalid_lex
                .iter()
                .map(|test| test.generate_test_case(stage))
                .reduce(|mut acc, s| {
                    acc.push_str(&s);
                    acc
                })
                .unwrap();

            let file_contents = format!("{}\n{}", imports, test_cases);

            let output_file = format!("chapter_{}.rs", self.chapter);
            let mut output_path = output_dir.to_owned();
            output_path.push(output_file);

            std::fs::write(output_path, file_contents).unwrap();
        }
    }

    pub fn generate_invalid_parse_tests(&self, output_dir: &Path, stage: CompilationMode) {
        let imports = self.generate_imports();
        if let Some(invalid_parse) = &self.invalid_parse {
            let test_cases = invalid_parse
                .iter()
                .map(|test| test.generate_test_case(stage))
                .reduce(|mut acc, s| {
                    acc.push_str(&s);
                    acc
                })
                .unwrap();

            let file_contents = format!("{}\n{}", imports, test_cases);

            let output_file = format!("chapter_{}.rs", self.chapter);
            let mut output_path = output_dir.to_owned();
            output_path.push(output_file);

            std::fs::write(output_path, file_contents).unwrap();
        }
    }

    fn generate_imports(&self) -> String {
        // Have to import lexer, parser and compiler
        "use lex::Lexer;\n".to_string()
    }
}

impl From<PathBuf> for TestSuite {
    fn from(path: PathBuf) -> Self {
        TestSuite::from(<&Path>::from(&path))
    }
}

impl From<&Path> for TestSuite {
    fn from(path: &Path) -> TestSuite {
        assert!(path.is_dir());

        let dir_name = path.file_name().unwrap().to_str().unwrap();

        let chapter: u8 = dir_name
            .strip_prefix("chapter_")
            .expect("Expected dir with name chapter_<number>")
            .parse()
            .unwrap();
        println!("{chapter}");

        let read_dir = |path: &Path| path.read_dir().unwrap().map(|f| f.unwrap().path());

        let read_test_cases = |path: &Path, mode: CompilationMode| {
            let tests = read_dir(path);
            tests.filter_map(move |path| {
                if path.is_file() {
                    Some(Test {
                        source_file: path.to_owned(),
                    })
                } else {
                    eprintln!("Skipping dir {path:?}");
                    None
                }
            })
        };

        let dirs = read_dir(path);

        let mut valid: Option<Vec<Test>> = None;
        let mut invalid_lex: Option<Vec<Test>> = None;
        let mut invalid_parse: Option<Vec<Test>> = None;

        for dir in dirs {
            match dir.file_name().unwrap().to_str().unwrap() {
                // TODO what mode should this be in?
                "valid" => valid = Some(read_test_cases(&dir, CompilationMode::Lex).collect()),
                "invalid_lex" => {
                    invalid_lex = Some(read_test_cases(&dir, CompilationMode::Lex).collect())
                }
                "invalid_parse" => {
                    invalid_parse = Some(read_test_cases(&dir, CompilationMode::Parse).collect())
                }
                _ => eprintln!("Ignoring unknown dir: {dir:?}"),
            }
        }

        TestSuite {
            chapter,
            valid,
            invalid_lex,
            invalid_parse,
        }
    }
}
