use std::error::Error;
use std::fmt;

pub trait AlphaError: Error {
    fn error_type(&self) -> &'static str;

    fn at(self, line: usize, lexeme: impl Into<String>) -> Located<Self>
    where
        Self: Sized,
    {
        Located {
            error: self,
            location: ErrorLocation::Token {
                line,
                lexeme: lexeme.into(),
            },
        }
    }

    fn at_end(self, line: usize) -> Located<Self>
    where
        Self: Sized,
    {
        Located {
            error: self,
            location: ErrorLocation::End { line },
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("[line {}] {}: {error} {location}", .location.line(), .error.error_type())]
pub struct Located<E: AlphaError> {
    #[source]
    pub error: E,
    pub location: ErrorLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorLocation {
    Token { line: usize, lexeme: String },
    End { line: usize },
}

impl ErrorLocation {
    pub fn line(&self) -> usize {
        match self {
            Self::Token { line, .. } | Self::End { line } => *line,
        }
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Token { lexeme, .. } => write!(f, "at {lexeme:?}"),
            Self::End { .. } => write!(f, "at end"),
        }
    }
}
