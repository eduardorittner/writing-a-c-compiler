use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CodegenError {}

impl Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "codegen error")
    }
}

impl Error for CodegenError {}

pub type CodegenResult<T> = Result<T, CodegenError>;
