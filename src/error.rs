use core::fmt;
use core::str;
#[cfg(feature = "std")]
use std::error;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::vec::Vec;

use TokenType;


/// An XML parser errors.
#[derive(Debug)]
pub enum Error {
    /// An invalid token with an optional cause.
    InvalidToken(TokenType, TextPos, Option<StreamError>),

    /// An unexpected token.
    UnexpectedToken(TokenType, TextPos),

    /// An unknown token.
    UnknownToken(TextPos),
}

impl Error {
    /// Returns the error position.
    pub fn pos(&self) -> TextPos {
        match *self {
            Error::InvalidToken(_, pos, _) => pos,
            Error::UnexpectedToken(_, pos) => pos,
            Error::UnknownToken(pos) => pos,
        }
    }
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

#[cfg(feature = "std")]
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

    /// An invalid name.
    InvalidName,

    /// An invalid/unexpected character.
    ///
    /// The first byte is an actual one, the second one is expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidChar(u8, u8, TextPos),

    /// An invalid/unexpected character.
    ///
    /// Just like `InvalidChar`, but specifies multiple expected characters.
    InvalidCharMultiple(u8, &'static [u8], TextPos),

    /// An unexpected character instead of `"` or `'`.
    InvalidQuote(char, TextPos),

    /// An unexpected character instead of an XML space.
    ///
    /// Includes: `' ' \n \r \t &#x20; &#x9; &#xD; &#xA;`.
    InvalidSpace(char, TextPos),

    /// An unexpected string.
    ///
    /// The first string is an actual one, others - expected.
    ///
    /// We are using a single value to reduce the struct size.
    #[cfg(feature = "std")]
    InvalidString(Vec<String>, TextPos),

    /// An unexpected string.
    ///
    /// The byte string is the expected one.
    #[cfg(not(feature = "std"))]
    InvalidString(&'static [u8], TextPos),

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
            StreamError::InvalidChar(actual, expected, pos) => {
                write!(f, "expected '{}' not '{}' at {}",
                       expected as char, actual as char, pos)
            }
            StreamError::InvalidCharMultiple(actual, ref expected, pos) => {
                let mut expected_iter = expected.iter().peekable();

                write!(f, "expected ")?;
                while let Some(&c) = expected_iter.next() {
                    write!(f, "'{}'", c as char)?;
                    if expected_iter.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " not '{}' at {}", actual as char, pos)
            }
            StreamError::InvalidQuote(c, pos) => {
                write!(f, "expected quote mark not '{}' at {}", c, pos)
            }
            StreamError::InvalidSpace(c, pos) => {
                write!(f, "expected space not '{}' at {}", c, pos)
            }
            #[cfg(feature = "std")]
            StreamError::InvalidString(ref strings, pos) => {
                write!(f, "expected '{}' not '{}' at {}",
                       strings[1..].join("', '"), strings[0], pos)
            }
            #[cfg(not(feature = "std"))]
            StreamError::InvalidString(expected, pos) => {
                // Assume that all input `text` are valid UTF-8 strings, so unwrap is safe.
                let expected_str = str::from_utf8(expected).unwrap();
                write!(f, "expected '{}' at {}, but wasn't found",
                       expected_str, pos)
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

#[cfg(feature = "std")]
impl error::Error for StreamError {
    fn description(&self) -> &str {
        "an XML stream parsing error"
    }
}


/// Position in text.
///
/// Position indicates a row/line and a column in the original text. Starting from 1:1.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct TextPos {
    pub row: u32,
    pub col: u32,
}

impl TextPos {
    /// Constructs a new `TextPos`.
    ///
    /// Should not be invoked manually, but rather via `Stream::gen_text_pos`.
    pub fn new(row: u32, col: u32) -> TextPos {
        TextPos { row, col }
    }
}

impl fmt::Display for TextPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}
