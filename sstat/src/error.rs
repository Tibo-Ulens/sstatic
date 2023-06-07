use std::fmt::{Display, Formatter};

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
#[derive(Debug)]
pub enum ParseError {
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

impl Display for ParseError {
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
