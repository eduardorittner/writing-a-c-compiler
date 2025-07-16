use crate::Handle;
use crate::handle;
use crate::line::LineHandle;

handle!(Comment, CommentHandle);

/// Comment token
///
/// A multi-line block comment is parsed as one token
struct Comment {
    offset: u32,
    len: u32,
    lines: Vec<LineHandle>,
}
