use crate::range::Range;

use token_impl::TokenInfo;
// Re-exports TokenType so other modules can see it
pub use token_impl::TokenType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
    info: TokenInfo,
    id: GenericId,
}

/// Encapsulates TokenInfo, since it's internal fields are unsafe to access normally
mod token_impl {

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
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

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct TokenInfo {
        data: u8,
    }

    impl std::fmt::Debug for TokenInfo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "type: {:?} has_error: {:?}",
                self.ttype(),
                self.has_error()
            )
        }
    }

    // TODO implement display for TokenType and TokenInfo

    const ERROR_BIT: u8 = 7;
    const ERROR: u8 = 1 << ERROR_BIT;

    impl TokenInfo {
        /// Only the lexer can create new tokens
        pub(crate) fn new(ttype: TokenType, has_error: bool) -> TokenInfo {
            TokenInfo {
                data: (ttype as u8) | (has_error as u8) << ERROR_BIT,
            }
        }

        /// Whether a token was created during error recovery
        pub fn has_error(self) -> bool {
            self.data & ERROR == ERROR
        }

        /// TokenType associated with TokenInfo
        pub fn ttype(self) -> TokenType {
            // SAFETY: TokenInfos can only be constructed from valid TokenTypes
            unsafe { std::mem::transmute(self.data & !ERROR) }
        }
    }
}

mod generic_id {

    const BITS: u32 = 24;
    const MAX: u32 = (1 << 24) - 1;

    /// Generic ID which holds a u32
    #[repr(packed)]
    pub struct GenericId {
        id: [u8; 3],
    }

    impl Into<u32> for GenericId {
        fn into(self) -> u32 {
            ((self.id[2] as u32) << 16) | ((self.id[1] as u32) << 8) | (self.id[0] as u32)
        }
    }

    impl From<u32> for GenericId {
        fn from(value: u32) -> Self {
            assert!(value <= MAX);

            // Store as little-endian
            let low = value as u8;
            let mid = ((value >> 8) & 0xFF) as u8;
            let high = ((value >> 16) & 0xFF) as u8;

            GenericId {
                id: [low, mid, high],
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use generic_id::GenericId;

        #[test]
        fn zero() {
            let value = 0;
            assert_eq!(value, GenericId::from(0).into())
        }
    }
}
