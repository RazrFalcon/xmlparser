use std::fmt;
use std::error;

use {
    TokenType,
};


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
    ///
    /// The first byte is an actual one, others - expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidChar(Vec<u8>, TextPos),

    /// An unexpected character instead of `"` or `'`.
    InvalidQuote(char, TextPos),

    /// An unexpected character instead of an XML space.
    ///
    /// Includes: `' ' \n \r \t &#x20; &#x9; &#xD; &#xA;`.
    InvalidSpace(char, TextPos),

    /// An unexpected character instead of an XML space.
    ///
    /// The first string is an actual one, others - expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidString(Vec<String>, TextPos),

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
            StreamError::InvalidChar(ref chars, pos) => {
                // Vec<u8> -> Vec<String>
                let list: Vec<String> =
                    chars.iter().skip(1).map(|c| String::from_utf8(vec![*c]).unwrap()).collect();

                write!(f, "expected '{}' not '{}' at {}",
                       list.join("', '"), chars[0] as char, pos)
            }
            StreamError::InvalidQuote(c, pos) => {
                write!(f, "expected quote mark not '{}' at {}", c, pos)
            }
            StreamError::InvalidSpace(c, pos) => {
                write!(f, "expected space not '{}' at {}", c, pos)
            }
            StreamError::InvalidString(ref strings, pos) => {
                write!(f, "expected '{}' not '{}' at {}",
                       strings[1..].join("', '"), strings[0], pos)
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
    /// Should not be invoked manually, but rather via `Stream::gen_error_pos`.
    pub fn new(row: u32, col: u32) -> TextPos {
        TextPos { row, col }
    }
}

impl fmt::Display for TextPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}

#[test]
fn err_size_1() {
    assert!(::std::mem::size_of::<Error>() <= 64);
}

#[test]
fn err_size_2() {
    assert!(::std::mem::size_of::<StreamError>() <= 64);
}
