use std::fmt;

use {
    TokenType,
};


/// An XML parser errors.
#[derive(Fail, Debug)]
pub enum Error {
    /// An invalid token.
    #[fail(display = "invalid token '{}' at {}", _0, _1)]
    InvalidToken(TokenType, ErrorPos),

    /// An invalid token with cause.
    #[fail(display = "invalid token '{}' at {} cause {}", _0, _1, _2)]
    InvalidTokenWithCause(TokenType, ErrorPos, StreamError),

    /// An unexpected token.
    #[fail(display = "unexpected token '{}' at {}", _0, _1)]
    UnexpectedToken(TokenType, ErrorPos),

    /// An unknown token.
    #[fail(display = "unknown token at {}", _0)]
    UnknownToken(ErrorPos),
}


/// A specialized `Result` type where the error is hard-wired to [`Error`].
///
/// [`Error`]: enum.Error.html
pub type Result<T> = ::std::result::Result<T, Error>;


/// A stream parser errors.
#[derive(Fail, Debug)]
pub enum StreamError {
    /// The steam ended earlier than we expected.
    ///
    /// Should only appear on invalid input data.
    /// Errors in a valid XML should be handled by errors below.
    #[fail(display = "unexpected end of stream")]
    UnexpectedEndOfStream,

    /// An unknown token.
    #[fail(display = "invalid name token")]
    InvalidName,

    /// An invalid/unexpected character in the stream.
    #[fail(display = "expected '{}' not '{}' at {}", _1, _0, _2)]
    InvalidChar(char, String, ErrorPos),

    /// An invalid reference.
    #[fail(display = "invalid reference")]
    InvalidReference,

    /// An invalid ExternalID in DTD.
    #[fail(display = "invalid ExternalID")]
    InvalidExternalID,
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
