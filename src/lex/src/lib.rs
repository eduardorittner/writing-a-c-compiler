pub mod error;
pub mod range;
pub mod token;

pub use error::LexError;
pub use token::{Token, TokenType};

use crate::range::Range;

pub struct Lexer<'src> {
    source: &'src str,
    rest: &'src str,
    offset: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source,
            rest: source,
            offset: 0,
        }
    }

    fn skip_whitespace(&mut self) -> Option<char> {
        let mut chars = self.rest.chars();

        let mut c;
        loop {
            c = chars.next()?;

            if !c.is_whitespace() {
                break;
            }

            self.offset += c.len_utf8();
            self.rest = &self.rest[c.len_utf8()..];
        }

        Some(c)
    }

    /// Consumes an identifier or keyword
    fn consume_ident(&mut self) -> Result<Token<'src>, LexError> {
        let mut chars = self.rest.chars();

        let mut c = chars.next().expect("This function should only be called when we still have at least one (alphabetical) char in the input");
        let start = self.offset;

        loop {
            match chars.next() {
                Some(c) if matches!(c, 'A'..='Z' | 'a'..='z' | '1'..='9' | '_') => (),
                Some(c) if c.is_whitespace() => break,
                _ => return Err(LexError),
                None => break,
            };

            self.offset += c.len_utf8();
        }

        self.offset += c.len_utf8();
        self.rest = &self.source[self.offset..];

        let token_range = Range::new(self.source, start, self.offset);
        return Ok(Token {
            ttype: TokenType::Ident,
            source: token_range,
        });
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token<'src>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.skip_whitespace()?;

        let c_offset = self.offset;
        let c_str = &self.rest[..c.len_utf8()];

        // Updates relevant state and returns single char tokens
        let mut just = |ttype: TokenType| {
            self.offset += c_str.len();
            self.rest = &self.rest[c_str.len()..];
            Some(Ok(Token {
                ttype,
                source: Range::new(self.source, c_offset, c_offset + c_str.len()),
            }))
        };

        // TODO have to update offset and rest before returning
        match c {
            '(' => return just(TokenType::OpenParen),
            ')' => return just(TokenType::CloseParen),
            '{' => return just(TokenType::OpenBrace),
            '}' => return just(TokenType::CloseBrace),
            ';' => return just(TokenType::Semicolon),
            c if c.is_alphabetic() => return Some(self.consume_ident()),

            _ => (),
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::Lexer;
    use crate::Range;
    use crate::{Token, TokenType};

    #[test]
    fn single_char_tokens() {
        let source = "()";
        let mut lexer = Lexer::new(source);

        assert_eq!(
            Some(Ok(Token {
                ttype: TokenType::OpenParen,
                source: Range::new(source, 0, 1)
            })),
            lexer.next()
        );
        assert_eq!(
            Some(Ok(Token {
                ttype: TokenType::CloseParen,
                source: Range::new(source, 1, 2)
            })),
            lexer.next()
        );
    }

    #[test]
    fn basic_keywords() {
        let source = "main int void";
        let mut lexer = Lexer::new(source);

        assert_eq!(
            Some(Ok(Token {
                ttype: TokenType::Ident,
                source: Range::new(source, 0, 4)
            })),
            lexer.next()
        );

        assert_eq!(
            Some(Ok(Token {
                ttype: TokenType::Ident,
                source: Range::new(source, 5, 8)
            })),
            lexer.next()
        );

        assert_eq!(
            Some(Ok(Token {
                ttype: TokenType::Ident,
                source: Range::new(source, 9, 13)
            })),
            lexer.next()
        );
    }
}
