use std::fmt;

use xml::TokenType;

use {
    StreamError,
    StreamErrorKind,
};


error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        Stream(StreamError, StreamErrorKind) #[doc = "Stream errors"];
    }

    errors {
        /// An invalid token.
        InvalidToken(t: TokenType, pos: ErrorPos) {
            display("invalid token '{}' at {}", t, pos)
        }

        /// An unexpected token.
        UnexpectedToken(t: TokenType, pos: ErrorPos) {
            display("unexpected token '{}' at {}", t, pos)
        }

        /// An unknown token.
        UnknownToken(pos: ErrorPos) {
            display("unknown token at {}", pos)
        }
    }
}


/// Position of the error.
///
/// Position indicates row/line and column. Starting positions is 1:1.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ErrorPos {
    #[allow(missing_docs)]
    pub row: usize,
    #[allow(missing_docs)]
    pub col: usize,
}

impl ErrorPos {
    /// Constructs a new error position.
    pub fn new(row: usize, col: usize) -> ErrorPos {
        ErrorPos {
            row: row,
            col: col,
        }
    }
}

impl fmt::Display for ErrorPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}
