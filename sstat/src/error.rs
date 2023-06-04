use std::fmt::{Display, Formatter};

/// Any error produced during transpilation
#[derive(Debug)]
pub enum Error {
    /// Wrapper around [`std::io::Error`]
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    #[inline(always)]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
