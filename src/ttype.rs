struct GenericId([u8; 3]);

pub struct Token {
    info: TokenInfo,
    id: GenericId,
}

// TODO Token must be in a different module since TokenInfo must have private fields
impl Token {
    pub fn ttype(self) -> TokenType {
        self.info.ttype()
    }

    pub fn has_error(self) -> bool {
        self.info.has_error()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// Contains error flag and TokenType
///
/// SAFETY:
pub struct TokenInfo(u8);

#[repr(u8)]
pub enum TokenType {
    Semicolon,
    OpenParen,
}

const ERROR_SHIFT: usize = 7;

pub const ERROR: u8 = 1 << ERROR_SHIFT;

impl TokenInfo {
    pub fn new(ttype: TokenType, error: bool) -> TokenInfo {
        // SAFETY: Since we receive a TokenType, self.0 & !ERROR is always a valid TokenType
        TokenInfo((ttype as u8) | (error as u8) << ERROR_SHIFT)
    }

    pub fn has_error(self) -> bool {
        self.0 & ERROR != 0
    }

    pub fn ttype(self) -> TokenType {
        // SAFETY: we only construct tokens with a valid TokenType, and token types are never
        // mutated
        unsafe { std::mem::transmute(self.0 & !ERROR) }
    }
}

#[cfg(test)]
mod static_asserts {
    use crate::ttype::{Token, TokenInfo, TokenType};

    #[test]
    fn ttype_variants() {
        assert!(128 > std::mem::variant_count::<TokenType>());
        assert_eq!(std::mem::size_of::<TokenInfo>(), 1);
        assert_eq!(std::mem::size_of::<TokenType>(), 1);
        assert_eq!(std::mem::size_of::<Token>(), 4);
    }
}
