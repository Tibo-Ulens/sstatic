use std::fmt::{Display, Formatter};

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;

use crate::parse::Span;

/// Any error produced during transpilation
#[derive(Debug)]
pub enum Error {
    /// Wrapper around [`std::io::Error`]
    Io(std::io::Error),
    /// Wrapper around [`ParseError`]
    ParseError(ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::ParseError(e) => write!(f, "{}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    #[inline(always)]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ParseError> for Error {
    #[inline(always)]
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}

/// Any error related to parsing
#[derive(Clone, Debug)]
pub struct ParseError {
    /// The source of the error
    pub source: Box<SimpleFile<String, String>>,
    /// The location of the error
    pub span: Span,
    /// The type of error
    pub ty: ParseErrorType,
    /// Any additional context about the error
    pub context: Vec<String>,
}

impl ParseError {
    /// Create a new error
    pub fn new(source: SimpleFile<String, String>, span: Span, ty: ParseErrorType) -> Self {
        Self {
            source: Box::new(source),
            span,
            ty,
            context: vec![],
        }
    }

    /// Add context to the error
    pub fn add_context(mut self, ctx: String) -> Self {
        self.context.push(ctx);

        self
    }

    fn as_diagnostic(&self) -> Diagnostic<()> {
        Diagnostic::error()
            .with_message(self.ty.message())
            .with_labels(vec![
                Label::primary((), self.span).with_message(self.ty.to_string())
            ])
            .with_notes(self.context.clone())
    }

    fn emit(&self, writer: &mut dyn codespan_reporting::term::termcolor::WriteColor) -> () {
        codespan_reporting::term::emit(
            writer,
            &Default::default(),
            self.source.as_ref(),
            &self.as_diagnostic(),
        )
        .unwrap()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = Vec::new();
        {
            let mut writer = codespan_reporting::term::termcolor::Ansi::new(&mut buffer);

            self.emit(&mut writer);
        }

        write!(f, "{}", std::str::from_utf8(&buffer).unwrap())
    }
}

/// All possible types of error encountered during parsing
#[derive(Clone, Debug)]
pub enum ParseErrorType {
    /// Unexpected end-of-file
    RawUnexpectedEof,
    /// Unexpected end-of-file
    #[allow(missing_docs)]
    UnexpectedEof { expected: String },
    /// Unexpected token
    #[allow(missing_docs)]
    UnexpectedToken { expected: String, found: String },
    /// Expected an identifier
    #[allow(missing_docs)]
    ExpectedIdentifier { found: String },
}

impl ParseErrorType {
    fn message(&self) -> String {
        match self {
            Self::RawUnexpectedEof => String::from("unexpected end-of-file"),
            Self::UnexpectedEof { expected: _ } => String::from("unexpected end-of-file"),
            Self::UnexpectedToken {
                expected: _,
                found: _,
            } => String::from("unexpected token"),
            Self::ExpectedIdentifier { found: _ } => String::from("expected identifier"),
        }
    }
}

impl Display for ParseErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RawUnexpectedEof => write!(f, "unexpected end-of-file"),
            Self::UnexpectedEof { expected } => {
                write!(f, "unexpected end-of-file, expected '{expected}'")
            }
            Self::UnexpectedToken { expected, found } => {
                write!(f, "unexpected token '{found}', expected '{expected}'")
            }
            Self::ExpectedIdentifier { found } => {
                write!(f, "expected IDENTIFIER, found '{found}'")
            }
        }
    }
}
