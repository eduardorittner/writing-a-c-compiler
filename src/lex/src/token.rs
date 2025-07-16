use crate::range::Range;

use generic_handle::HandleData;
use token_impl::TokenInfo;

// Re-exports TokenType so other modules can see it
pub use generic_handle::{HANDLE_MAX, Handle};
pub use token_impl::TokenType;

/// Creates a new handle with the given name
#[macro_export]
macro_rules! handle {
    ($type:ident, $handleName:ident) => {
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

        impl<$type> Handle<$type> for $handleName {}
    };
}

/// Small token which only has the token type, error flag and a handle for related information such
/// as position in the source text, line, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    info: TokenInfo,
    id: HandleData,
}

handle!(Token, TokenHandle);

/// Has a handle to the token's original source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSource {
    start: u32,
    end: u32,
    line: u32,
    // NOTE: We have 4 bytes to spare
    // could have a handle to interned values for numeric constants
}

handle!(TokenSource, TokenSourceHandle);

#[cfg(test)]
mod test {
    use super::Token;

    #[test]
    fn token_info_size() {
        assert_eq!(4, std::mem::size_of::<Token>());
    }
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

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn size_asserts() {
            assert_eq!(std::mem::size_of::<TokenType>(), 1);
            assert_eq!(std::mem::size_of::<TokenInfo>(), 1);
        }
    }
}

mod generic_handle {

    pub trait Handle<V>: From<u32> + Into<u32> {}

    // This is 3 bytes since we want `Token` to only be 4 bytes, it this need ever changes we can up
    // this to u32 or u64. 8 Million of anything is plenty enough (probably), since to have 8
    // million tokens in one `ValueStore` we would need a file *at least* larger than 8MB, which is
    // pretty big. Maybe this won't hold for ast nodes?
    const HANDLE_BITS: u32 = 24;
    pub const HANDLE_MAX: u32 = (1 << HANDLE_BITS) - 1;

    /// Handle which holds a u32
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(packed)]
    pub struct HandleData {
        id: [u8; 3],
    }

    impl From<HandleData> for usize {
        fn from(value: HandleData) -> Self {
            u32::from(value) as usize
        }
    }

    impl From<usize> for HandleData {
        fn from(value: usize) -> Self {
            (value as u32).into()
        }
    }

    impl From<HandleData> for u32 {
        fn from(value: HandleData) -> Self {
            ((value.id[2] as u32) << 16) | ((value.id[1] as u32) << 8) | (value.id[0] as u32)
        }
    }

    impl From<u32> for HandleData {
        fn from(value: u32) -> Self {
            assert!(value <= HANDLE_MAX);

            // Store as little-endian
            let low = value as u8;
            let mid = ((value >> 8) & 0xFF) as u8;
            let high = ((value >> 16) & 0xFF) as u8;

            HandleData {
                id: [low, mid, high],
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[macro_export]

        // Taken from assert_panic crate
        macro_rules! assert_panic {
            ($stmt:stmt$(,)?) => {
                ::std::panic::catch_unwind(|| -> () { $stmt })
                    .expect_err("assert_panic! argument did not panic")
            };
        }

        #[test]
        fn zero() {
            let value: u32 = 0;
            assert_eq!(value, HandleData::from(value).into())
        }

        #[test]
        fn one() {
            let value: u32 = 1;
            assert_eq!(value, HandleData::from(value).into())
        }

        #[test]
        fn u24_max() {
            let value: u32 = (1 << 24) - 1;
            assert_eq!(value, HandleData::from(value).into())
        }

        #[test]
        fn u24_max_plus_1() {
            let value: u32 = 1 << 24;
            assert_panic!({
                let _ = HandleData::from(value);
            });
        }

        #[test]
        fn u32_max() {
            let value: u32 = u32::MAX;
            assert_panic!({
                let _ = HandleData::from(value);
            });
        }
    }
}
