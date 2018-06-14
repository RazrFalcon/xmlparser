use std::fmt;
use std::error;

use {
    TokenType,
};


/// An XML parser errors.
#[derive(Debug)]
pub enum Error {
    /// An invalid token with an optional cause.
    InvalidToken(TokenType, ErrorPos, Option<StreamError>),

    /// An unexpected token.
    UnexpectedToken(TokenType, ErrorPos),

    /// An unknown token.
    UnknownToken(ErrorPos),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidToken(token_type, pos, ref cause) => {
                match *cause {
                    Some(ref cause) => {
                        write!(f, "invalid token '{}' at {} cause {}", token_type, pos, cause)
                    }
                    None => {
                        write!(f, "invalid token '{}' at {}", token_type, pos)
                    }
                }
            }
            Error::UnexpectedToken(token_type, pos) => {
                write!(f, "unexpected token '{}' at {}", token_type, pos)
            }
            Error::UnknownToken(pos) => {
                write!(f, "unknown token at {}", pos)
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "an XML parsing error"
    }
}


/// A stream parser errors.
#[derive(Debug)]
pub enum StreamError {
    /// The steam ended earlier than we expected.
    ///
    /// Should only appear on invalid input data.
    /// Errors in a valid XML should be handled by errors below.
    UnexpectedEndOfStream,

    /// An unknown token.
    InvalidName,

    /// An invalid/unexpected character.
    InvalidChar(char, String, ErrorPos),

    /// An unexpected character instead of `"` or `'`.
    InvalidQuote(char, ErrorPos),

    /// An invalid reference.
    InvalidReference,

    /// An invalid ExternalID in the DTD.
    InvalidExternalID,
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StreamError::UnexpectedEndOfStream => {
                write!(f, "unexpected end of stream")
            }
            StreamError::InvalidName => {
                write!(f, "invalid name token")
            }
            StreamError::InvalidChar(c, ref s, pos) => {
                write!(f, "expected '{}' not '{}' at {}", s, c, pos)
            }
            StreamError::InvalidQuote(c, pos) => {
                write!(f, "expected quote mark not '{}' at {}", c, pos)
            }
            StreamError::InvalidReference => {
                write!(f, "invalid reference")
            }
            StreamError::InvalidExternalID => {
                write!(f, "invalid ExternalID")
            }
        }
    }
}

impl error::Error for StreamError {
    fn description(&self) -> &str {
        "an XML stream parsing error"
    }
}


/// Position of the error.
///
/// Position indicates row/line and column. Starting positions is 1:1.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct ErrorPos {
    pub row: usize,
    pub col: usize,
}

impl ErrorPos {
    /// Constructs a new error position.
    pub fn new(row: usize, col: usize) -> ErrorPos {
        ErrorPos { row, col }
    }
}

impl fmt::Display for ErrorPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}
