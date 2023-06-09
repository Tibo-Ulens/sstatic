//! AST types for location info

use std::path::PathBuf;

/// All information about the location of a specific item
#[derive(Clone, Debug)]
pub struct Location {
    /// The location of the file in which the item occurs
    pub file_path: PathBuf,
    /// The lines of source code encompassing this item
    pub lines: Vec<String>,
    /// The location of the start of this item
    pub start: LineCol,
    /// The location of the end of this item
    pub end: LineCol,
    /// The region of source code spanned by this item
    pub span: Span,
}

/// Information about where a given AST node is located in the source file
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineCol {
    /// The line on which the node starts
    pub line: usize,
    /// The column on which node starts
    pub column: usize,
}

/// A specific region of source code
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// The start of the span
    pub start: usize,
    /// The end of the span
    pub end: usize,
}

impl From<Span> for std::ops::Range<usize> {
    fn from(value: Span) -> Self {
        value.start..value.end
    }
}

impl Span {
    /// Make a new [`Span`]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
