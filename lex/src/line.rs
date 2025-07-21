use crate::handle;

handle!(LineHandle);

/// Line information
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: usize,
    pub end: usize,
}
