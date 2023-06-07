//! AST type definitions

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use unicode_xid::UnicodeXID;

mod location;
mod node;
mod token;

pub(crate) use location::*;
pub(crate) use node::*;
pub(crate) use token::*;

use crate::{Error, ParseError};

type ParseResult<'i, T> = Result<(&'i str, (T, Span)), ErrorKind<Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ErrorKind<E> {
    Error(E),
    Failure(E),
}

thread_local! {
    /// A thread local [`FileInfo`] struct for the current file
    static FILE_INFO: RefCell<FileInfo> = RefCell::new(FileInfo {
        file_name: String::new(),
        file_path: PathBuf::new(),
        file_src: String::new(),
        lines: Vec::new(),
    });
}

/// All of the information about a specific file
#[derive(Clone, Debug)]
pub(crate) struct FileInfo {
    /// The name of the file
    file_name: String,
    /// The path to the file
    file_path: PathBuf,
    /// The source code of the file
    file_src: String,
    /// The start index of each line
    lines: Vec<usize>,
}

impl FileInfo {
    /// Create a new [`FileInfo`] struct
    pub(crate) fn new(file_name: &str, file_path: &Path, file_src: &str) -> Self {
        let mut lines = vec![0];
        let mut total = 0;
        for ch in file_src.chars() {
            total += 1;
            if ch == '\n' {
                lines.push(total);
            }
        }

        Self {
            file_name: file_name.to_owned(),
            file_path: file_path.to_owned(),
            file_src: file_src.to_owned(),
            lines,
        }
    }

    /// Initialize the thread local [`FileInfo`] struct
    pub(crate) fn init_thread_local(self) {
        FILE_INFO.with(|f| {
            *f.borrow_mut() = self;
        });
    }

    /// Get the [`Location`] in the source file given an offset from the start
    pub(crate) fn offset_location(&self, offset: usize) -> Location {
        match self.lines.binary_search(&offset) {
            Ok(idx) => Location {
                line: idx + 1,
                column: 0,
            },
            Err(idx) => Location {
                line: idx,
                column: offset - self.lines[idx - 1],
            },
        }
    }
}

pub(crate) struct Parser;

impl Parser {
    /// Keep applying a given combinator as long as it succeeds
    fn many<'i, O, F>(combinator: F) -> impl Fn(&'i str, usize) -> ParseResult<Vec<(O, Span)>>
    where
        F: Fn(&'i str, usize) -> ParseResult<O>,
    {
        move |mut input: &str, start: usize| {
            let mut span = Span::new(start, start);
            let mut values = vec![];

            loop {
                match combinator(input, span.end) {
                    Ok((rest, (o, o_span))) => {
                        // Update `span` to point at the end of the most
                        // recently parsed value
                        span.end = o_span.end;

                        values.push((o, o_span));

                        input = rest;
                    }
                    Err(ErrorKind::Error(_)) => return Ok((input, (values, span))),
                    Err(e) => return Err(e),
                }
            }
        }
    }

    /// Match an exact tag
    fn tag(tag: &str) -> impl Fn(&str, usize) -> ParseResult<&str> + '_ {
        move |input: &str, start: usize| {
            let tag_len = tag.len();
            if tag_len > input.len() {
                return Err(ErrorKind::Failure(
                    ParseError::UnexpectedEof {
                        expected: tag.to_owned(),
                    }
                    .into(),
                ));
            }

            let (i_tag, rest) = input.split_at(tag_len);

            if i_tag == tag {
                let span = Span::new(start, start + tag_len);
                return Ok((rest, (i_tag, span)));
            }

            Err(ErrorKind::Error(
                ParseError::UnexpectedToken {
                    expected: tag.to_owned(),
                    found: i_tag.to_owned(),
                }
                .into(),
            ))
        }
    }

    /// Returns the longest input slice that matches the predicate
    fn take_while<F>(pred: F) -> impl Fn(&str, usize) -> ParseResult<&str>
    where
        F: Fn(char) -> bool,
    {
        move |input: &str, start: usize| match input.find(|c| !pred(c)) {
            Some(i) => {
                let rest = unsafe { input.get_unchecked(i..) };
                let taken = unsafe { input.get_unchecked(..i) };
                let span = Span::new(start, start + i);

                Ok((rest, (taken, span)))
            }
            None => {
                let rest = unsafe { input.get_unchecked(input.len()..) };
                let taken = unsafe { input.get_unchecked(..input.len()) };
                let span = Span::new(start, start + input.len());

                Ok((rest, (taken, span)))
            }
        }
    }

    /// Helper function that consumes all leading whitespace and/or comments
    fn take_non_parseable() -> impl Fn(&str, usize) -> ParseResult<()> {
        move |input: &str, start: usize| {
            let (rest, (_, span)) = Self::many(Self::take_comment())(input, start)?;

            Ok((rest, ((), span)))
        }
    }

    /// Take a comment along with its leading whitespace
    fn take_comment() -> impl Fn(&str, usize) -> ParseResult<()> {
        move |mut input: &str, start: usize| {
            let mut span = Span { start, end: start };

            // Consume any leading whitespace
            match Self::take_while(|c| c.is_ascii_whitespace())(input, span.end) {
                Ok((rest, (_, s))) => {
                    if s.end - s.start < 1 {
                        return Err(ErrorKind::Error(ParseError::RawUnexpectedEof.into()));
                    }

                    input = rest;
                    span.end = s.end;
                }
                Err(_) => unreachable!(),
            };

            // Check for comments
            match Self::tag(";;")(input, span.end) {
                Ok((rest, (_, s))) => {
                    input = rest;
                    span.end = s.end;
                }
                Err(_) => return Ok((input, ((), span))),
            }

            // If there is a comment, consume till end-of-line
            match Self::take_while(|c| c != '\n')(input, span.end) {
                Ok((rest, (_, s))) => {
                    input = rest;
                    span.end = s.end;
                }
                Err(_) => unreachable!(),
            };

            Ok((input, ((), span)))
        }
    }
}
