use crate::Handle;
use crate::handle;

handle!(Line, LineHandle);

/// Line information
struct Line {
    offset: u32,
    length: u32,
}
