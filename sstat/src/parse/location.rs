//! AST location info types and helpers

use crate::parse::FILE_INFO;

/// Information about where a given AST node is located in the source file
#[derive(Clone, Copy, Debug)]
pub(crate) struct Location {
    /// The line on which the node starts
    pub(crate) line: usize,
    /// The column on which node starts
    pub(crate) column: usize,
}

/// A specific region of source code
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Span {
    /// The start of the span
    pub(crate) start: usize,
    /// The end of the span
    pub(crate) end: usize,
}

impl Span {
    pub(crate) fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Get the [`Location`] of the start of this [`Span`]
    pub(crate) fn start(&self) -> Location {
        FILE_INFO.with(|fi| {
            let fi = fi.borrow();
            fi.offset_location(self.start)
        })
    }

    /// Get the [`Location`] of the end of this [`Span`]
    pub(crate) fn end(&self) -> Location {
        FILE_INFO.with(|fi| {
            let fi = fi.borrow();
            fi.offset_location(self.end)
        })
    }
}
