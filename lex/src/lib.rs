pub mod line;
pub mod token;

pub use token::{Token, TokenType};
use tracing::{Level, span};

use crate::{line::Line, token::TokenSource};

pub use output::TokenizedOutput;

pub struct Lexer<'src> {
    source: &'src str,
    rest: &'src str,
    offset: usize,
    output: TokenizedOutput<'src>,
}

impl Lexer<'_> {
    pub fn new(source: &str) -> Lexer<'_> {
        Lexer {
            source,
            rest: source,
            offset: 0,
            output: TokenizedOutput::new(source),
        }
    }

    /// Skips whitespace and comments, updating line info on every newline encountered
    fn skip_whitespace(&mut self) -> Option<char> {
        let mut chars = self.rest.chars();
        let mut c;

        loop {
            c = self.skip_comments()?;

            if !c.is_whitespace() {
                break Some(c);
            };

            self.offset += c.len_utf8();

            if c == '\n' {
                let line = if let Some(last_line) = self.output.last_line() {
                    Line {
                        start: last_line.end,
                        end: self.offset,
                    }
                } else {
                    Line {
                        start: 0,
                        end: self.offset,
                    }
                };

                self.output.push_line(line);
            }

            self.rest = &self.rest[c.len_utf8()..];
        }
    }

    /// Skips comments
    fn skip_comments(&mut self) -> Option<char> {
        let mut chars = self.rest.chars();
        let mut c = chars.next()?;
        let mut in_comment = false;
        let mut in_multiline_comment = false;

        if c == '/' {
            match chars.next() {
                Some('/') => in_comment = true,
                Some('*') => in_multiline_comment = true,
                Some(_) | None => (),
            }
        } else {
            return Some(c);
        }

        self.offset += c.len_utf8() * 2;
        self.rest = &self.rest[c.len_utf8() * 2..];

        if in_comment {
            while let Some(c) = chars.next() {
                self.offset += c.len_utf8();
                self.rest = &self.rest[c.len_utf8()..];
                if c == '\n' {
                    break;
                }
            }
        } else if in_multiline_comment {
            while let Some(c) = chars.next() {
                self.offset += c.len_utf8();
                self.rest = &self.rest[c.len_utf8()..];
                if c == '*' {
                    if let Some(c) = chars.next() {
                        self.offset += c.len_utf8();
                        self.rest = &self.rest[c.len_utf8()..];
                        if c == '/' {
                            break;
                        }
                    }
                }
            }
        }
        chars.next()
    }

    /// Consumes an identifier
    fn consume_ident(&mut self) {
        let mut chars = self.rest.chars();

        let c = chars.next().expect("This function should only be called when we still have at least one (alphabetical) char in the input");
        let start = self.offset;

        loop {
            match chars.next() {
                Some('A'..='Z' | 'a'..='z' | '1'..='9' | '_') => (),
                Some(c) if c.is_whitespace() => break,
                _ => break,
            };
            self.offset += c.len_utf8();
        }

        self.offset += c.len_utf8();
        self.rest = &self.source[self.offset..];

        let token_source = TokenSource {
            start,
            end: self.offset,
            line: self.output.current_line(),
        };

        self.output
            .push_token(TokenType::Ident, false, token_source);
    }

    fn consume_numeric_constant(&mut self) {
        let mut chars = self.rest.chars();

        let c = chars.next().expect("This function should only be called when we still have at least one (alpha_numerical) char in the input");
        let start = self.offset;

        // We allow '_' inside numbers
        while let Some('0'..='9' | '_') = chars.next() {
            self.offset += '0'.len_utf8();
        }

        self.offset += c.len_utf8();

        self.rest = &self.source[self.offset..];

        let token_source = TokenSource {
            start,
            end: self.offset,
            line: self.output.current_line(),
        };

        self.output
            .push_token(TokenType::Constant, false, token_source);
    }

    pub fn lex(source: &str) -> TokenizedOutput<'_> {
        let _ = span!(Level::TRACE, "Lexing").entered();

        let mut lexer = Self::new(source);

        lexer.run_lexer();
        lexer.output
    }

    fn run_lexer(&mut self) {
        while let Some(c) = self.skip_whitespace() {
            let c_offset = self.offset;
            let c_str = &self.rest[..c.len_utf8()];
            let mut emit_single_char_token = |ttype: TokenType| {
                self.offset += c_str.len();
                self.rest = &self.rest[c_str.len()..];
                self.output.push_token(
                    ttype,
                    false,
                    TokenSource {
                        start: c_offset,
                        end: c_offset + c.len_utf8(),
                        line: self.output.current_line(),
                    },
                );
            };

            match c {
                '(' => emit_single_char_token(TokenType::OpenParen),
                ')' => emit_single_char_token(TokenType::CloseParen),
                '{' => emit_single_char_token(TokenType::OpenBrace),
                '}' => emit_single_char_token(TokenType::CloseBrace),
                ';' => emit_single_char_token(TokenType::Semicolon),
                '/' => emit_single_char_token(TokenType::FrontSlash),
                '\\' => emit_single_char_token(TokenType::BackSlash),
                '-' => emit_single_char_token(TokenType::Hyphen),
                ':' => emit_single_char_token(TokenType::Colon),
                '*' => emit_single_char_token(TokenType::Asterisk),
                '\'' => emit_single_char_token(TokenType::Quote),
                '"' => emit_single_char_token(TokenType::DoubleQuote),
                ',' => emit_single_char_token(TokenType::Comma),
                'a'..='z' | 'A'..='Z' | '_' => self.consume_ident(),
                c if c.is_ascii_alphanumeric() => self.consume_numeric_constant(),
                '\0' => break,
                _ => panic!("{c:?}"),
            };
        }
        // Emit last line since it doesn't (necessarily) have a '\n'
        self.output.push_line(Line {
            start: self.output.current_line_offset(),
            end: self.offset,
        });
    }
}

mod output {
    use std::fmt::{self, Display};

    use crate::{Token, TokenType, line::Line, token::TokenSource};

    #[derive(Debug)]
    pub struct TokenizedOutput<'src> {
        source: &'src str,
        tokens: Vec<Token>,
        token_sources: Vec<TokenSource>,
        // We don't need a Vec<LineHandle> since they're a simple range [0..lines.len()]
        lines: Vec<Line>,
    }

    impl<'src> TokenizedOutput<'src> {
        pub(crate) fn new(source: &'src str) -> Self {
            TokenizedOutput {
                source,
                tokens: Vec::new(),
                token_sources: Vec::new(),
                lines: Vec::new(),
            }
        }

        pub fn len(&self) -> usize {
            self.tokens.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn get(&self, index: usize) -> Option<Token> {
            self.tokens.get(index).copied()
        }

        pub fn token_source(&self, handle: usize) -> TokenSource {
            *self.token_sources.get(handle).unwrap()
        }

        pub fn token_text(&self, handle: usize) -> &'src str {
            self.token_source(handle).fmt(self.source)
        }

        pub fn tokens(&self) -> &[Token] {
            &self.tokens
        }

        pub(crate) fn push_token(
            &mut self,
            ttype: TokenType,
            has_error: bool,
            source: TokenSource,
        ) {
            self.tokens.push(Token {
                ttype,
                has_error,
                handle: self.token_sources.len(),
            });
            self.token_sources.push(source);
        }

        pub(crate) fn push_line(&mut self, line: Line) {
            self.lines.push(line);
        }

        pub(crate) fn last_line(&self) -> Option<Line> {
            self.lines.last().copied()
        }

        pub(crate) fn current_line(&self) -> usize {
            self.lines.len()
        }

        pub(crate) fn current_line_offset(&self) -> usize {
            if let Some(line) = self.last_line() {
                line.end
            } else {
                0
            }
        }
    }

    impl<'src> Display for TokenizedOutput<'src> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (token, source) in self.tokens.iter().zip(self.token_sources.iter()) {
                writeln!(
                    f,
                    "{}: [{}] \"{}\"",
                    source.line,
                    token.ttype,
                    source.fmt(self.source)
                )?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Lexer, TokenType};

    macro_rules! snapshot_test (
        ($string:expr) => {
            insta::assert_debug_snapshot!(Lexer::lex($string));
        };
    );

    #[test]
    fn empty_input() {
        snapshot_test!("");
    }

    #[test]
    fn single_char_tokens() {
        snapshot_test!("(){};(");
    }

    #[test]
    fn single_char_tokens_whitespace() {
        snapshot_test!("( ) { } ; (");
    }

    #[test]
    fn single_char_tokens_multiple_lines() {
        snapshot_test!("(\n)\n{\n}\n;\n\n\n");
    }

    #[test]
    fn simple_ident() {
        snapshot_test!("ident");
    }

    #[test]
    fn multiple_simple_ident() {
        snapshot_test!("ident main func int hi");
    }

    #[test]
    fn complex_idents() {
        snapshot_test!(
            "__underscores __more_under_scores_ some1number234 _under1_score_2_with3_numbers5"
        );
    }

    #[test]
    fn i64_max() {
        let source = format!("{}", i64::MAX);
        let mut lexer = Lexer::new(&source);

        lexer.run_lexer();

        let output = lexer.output;
        assert_eq!(output.len(), 1);

        let token = output.get(0).unwrap();
        assert_eq!(&source, output.token_source(token.handle).fmt(&source));
    }
}
