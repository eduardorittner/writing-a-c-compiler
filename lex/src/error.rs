use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum LexError {
    InvalidChar { c: char },
    InvalidNumericConstant { c: char },
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::InvalidChar { c } => write!(f, "Invalid char '{c}'"),
            LexError::InvalidNumericConstant { c } => {
                write!(f, "Invalid char in numeric constant '{c}'")
            }
        }
    }
}

impl Error for LexError {}

pub type LexResult<T> = Result<T, LexError>;
