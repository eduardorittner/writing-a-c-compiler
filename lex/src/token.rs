use std::fmt::Display;

/// Creates a new handle with the given name
#[macro_export]
macro_rules! handle {
    ($handleName:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $handleName(u32);

        impl From<u32> for $handleName {
            fn from(value: u32) -> Self {
                Self(value)
            }
        }

        impl From<$handleName> for u32 {
            fn from(value: $handleName) -> u32 {
                value.0
            }
        }
    };
}

/// Small token which only has the token type, error flag and a handle for related information such
/// as position in the source text, line, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub ttype: TokenType,
    pub has_error: bool,
    pub handle: usize,
}

handle!(TokenHandle);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSource {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl<'src> TokenSource {
    pub fn fmt(&self, source: &'src str) -> &'src str {
        &source[self.start..self.end]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Ident,
    Keyword,
    Constant,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Colon,       // ':'
    FrontSlash,  // '/'
    BackSlash,   // '\'
    Hyphen,      // '-'
    Asterisk,    // '*'
    Quote,       // "'"
    DoubleQuote, // '"'
    Comma,       // ','
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Ident => write!(f, "Ident"),
            TokenType::Keyword => todo!(),
            TokenType::Constant => write!(f, "Constant"),
            TokenType::OpenParen => write!(f, "OpenParen"),
            TokenType::CloseParen => write!(f, "CloseParen"),
            TokenType::OpenBrace => write!(f, "OpenBrace"),
            TokenType::CloseBrace => write!(f, "CloseBrace"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::FrontSlash => write!(f, "FrontsSlash"),
            TokenType::Hyphen => write!(f, "Hyphen"),
            TokenType::BackSlash => write!(f, "BackSlash"),
            TokenType::Asterisk => write!(f, "Asterisk"),
            TokenType::Quote => write!(f, "Quote"),
            TokenType::DoubleQuote => write!(f, "DoubleQuote"),
            TokenType::Comma => write!(f, "Comma"),
        }
    }
}
