//! AST type definitions

use std::path::{Path, PathBuf};

use codespan_reporting::files::SimpleFile;
use unicode_xid::UnicodeXID;

mod location;
mod node;
mod token;

pub use location::*;
pub(crate) use node::*;
pub(crate) use token::*;

use crate::{ParseError, ParseErrorType};

type ParseResult<'i, T> = Result<(&'i str, (T, Span)), ErrorKind<ParseError>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ErrorKind<E> {
    Error(E),
    Failure(E),
}

/// Wrapper around all the information needed for parsing
#[derive(Clone)]
pub(crate) struct Parser {
    file: SimpleFile<String, String>,
}

impl Parser {
    /// Make a new [`Parser`]
    pub(crate) fn new(file: SimpleFile<String, String>) -> Self {
        Self { file }
    }

    fn make_error(&self, span: Span, ty: ParseErrorType) -> ParseError {
        ParseError::new(self.file.clone(), span, ty)
    }

    /// Keep applying a given combinator as long as it succeeds
    fn many<'i, O, F>(
        &self,
        combinator: F,
    ) -> impl Fn(&'i str, usize) -> ParseResult<Vec<(O, Span)>>
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
                    Err(ErrorKind::Failure(_)) => return Ok((input, (values, span))),
                }
            }
        }
    }

    /// Attempts to apply a combinator, returning [`None`] on failure
    fn optional<'i, O, F>(&self, combinator: F) -> impl Fn(&'i str, usize) -> ParseResult<Option<O>>
    where
        F: Fn(&'i str, usize) -> ParseResult<O>,
    {
        move |input: &str, start: usize| match combinator(input, start) {
            Ok((rest, (o, span))) => Ok((rest, (Some(o), span))),
            Err(_) => Ok((input, (None, Span::new(start, start)))),
        }
    }

    /// Match an exact tag
    fn tag<'p, 'i: 'p>(
        &'p self,
        tag: &'i str,
    ) -> impl Fn(&'i str, usize) -> ParseResult<&'i str> + '_ {
        move |input: &str, start: usize| {
            let tag_len = tag.len();
            if tag_len > input.len() {
                return Err(ErrorKind::Failure(self.make_error(
                    Span::new(start, start + input.len()),
                    ParseErrorType::UnexpectedEof {
                        expected: tag.to_owned(),
                    },
                )));
            }

            let (i_tag, rest) = input.split_at(tag_len);

            if i_tag == tag {
                let span = Span::new(start, start + tag_len);
                return Ok((rest, (i_tag, span)));
            }

            Err(ErrorKind::Error(self.make_error(
                Span::new(start, start + tag_len),
                ParseErrorType::UnexpectedToken {
                    expected: tag.to_owned(),
                    found: i_tag.to_owned(),
                },
            )))
        }
    }

    /// Returns the longest input slice that matches the predicate
    fn take_while<F>(&self, pred: F) -> impl Fn(&str, usize) -> ParseResult<&str>
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
    fn take_non_parseable(&self) -> impl Fn(&str, usize) -> ParseResult<()> + '_ {
        move |input: &str, start: usize| {
            let (rest, (_, span)) = self.many(self.take_comment())(input, start)?;

            Ok((rest, ((), span)))
        }
    }

    /// Take a comment along with its leading whitespace
    fn take_comment(&self) -> impl Fn(&str, usize) -> ParseResult<()> + '_ {
        move |mut input: &str, start: usize| {
            let mut span = Span { start, end: start };

            // Consume any leading whitespace
            match self.take_while(|c| c.is_ascii_whitespace())(input, span.end) {
                Ok((rest, (_, s))) => {
                    if s.end - s.start < 1 {
                        return Err(ErrorKind::Error(
                            self.make_error(s, ParseErrorType::RawUnexpectedEof),
                        ));
                    }

                    input = rest;
                    span.end = s.end;
                }
                Err(_) => unreachable!(),
            };

            // Check for comments
            match self.tag(";;")(input, span.end) {
                Ok((rest, (_, s))) => {
                    input = rest;
                    span.end = s.end;
                }
                Err(_) => return Ok((input, ((), span))),
            }

            // If there is a comment, consume till end-of-line
            match self.take_while(|c| c != '\n')(input, span.end) {
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
