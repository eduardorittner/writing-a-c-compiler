use crate::range::Range;

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
}
