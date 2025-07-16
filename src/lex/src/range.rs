/// A string slice
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range<'src> {
    start: usize,
    end: usize,
    source: &'src str,
}

impl<'src> Range<'src> {
    pub fn new(source: &str, start: usize, end: usize) -> Range {
        Range { start, end, source }
    }
}

impl<'src> Into<&'src str> for Range<'src> {
    fn into(self) -> &'src str {
        &self.source[self.start..self.end]
    }
}

impl<'src> std::fmt::Display for Range<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.source[self.start..self.end])
    }
}
