use std::fmt;

use {
    Stream,
};


/// An immutable string slice.
///
/// Unlike `&str` contains a reference to the original string
/// and a span position.
#[must_use]
#[derive(PartialEq, Clone, Copy)]
pub struct StrSpan<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

impl<'a> StrSpan<'a> {
    /// Constructs a new `StrSpan` from string.
    pub fn from_str(text: &str) -> StrSpan {
        StrSpan {
            text: text,
            start: 0,
            end: text.len(),
        }
    }

    /// Constructs a new `StrSpan` from substring.
    pub fn from_substr(text: &str, start: usize, end: usize) -> StrSpan {
        debug_assert!(start <= end);
        debug_assert!(text.is_char_boundary(start));
        debug_assert!(text.is_char_boundary(end));

        StrSpan {
            text: text,
            start: start,
            end: end,
        }
    }

    /// Returns a start position of the span.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns a end position of the span.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns a length of the span.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns a length of the span.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a length of the span underling string.
    pub fn full_len(&self) -> usize {
        self.text.len()
    }

    /// Returns a span slice.
    ///
    /// A bit expensive, since Rust checks for char boundary.
    pub fn to_str(&self) -> &'a str {
        &self.text[self.start..self.end]
    }

    /// Returns a span slice as bytes.
    ///
    /// The same as `to_str` but does not involve char boundary checking.
    pub fn as_bytes(&self) -> &'a [u8] {
        &self.text.as_bytes()[self.start..self.end]
    }

    /// Returns an underling string region as `StrSpan`.
    pub fn slice_region(&self, start: usize, end: usize) -> StrSpan<'a> {
        let start = self.start + start;
        let end = self.start + end;

        StrSpan::from_substr(self.text, start, end)
    }

    /// Returns an underling string.
    pub fn full_str(&self) -> &'a str {
        self.text
    }

    /// Returns a trimmed version of this `StrSpan`.
    ///
    /// This function trim escaped spaces (aka `&#x20;`) too.
    pub fn trim(&self) -> StrSpan<'a> {
        let mut s = Stream::from_span(*self);
        s.skip_spaces();

        let start = s.pos();
        let mut end;
        loop {
            s.skip_bytes(|s, _| !s.starts_with_space());
            end = s.pos();
            s.skip_spaces();

            if s.at_end() {
                break;
            }
        }

        self.slice_region(start, end)
    }
}

impl<'a> From<&'a str> for StrSpan<'a> {
    fn from(text: &'a str) -> Self {
        StrSpan::from_str(text)
    }
}

impl<'a> fmt::Debug for StrSpan<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StrSpan({:?} {}..{})", self.to_str(), self.start, self.end)
    }
}

impl<'a> fmt::Display for StrSpan<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}


/// A general tokenizer interface.
pub trait FromSpan<'a>
    where Self: Sized
{
    /// Constructs a new `Tokenizer` from a string.
    fn from_str(text: &'a str) -> Self {
        Self::from_span(StrSpan::from_str(text))
    }

    /// Constructs a new `Tokenizer` from `StrSpan`.
    fn from_span(span: StrSpan<'a>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_1() {
        assert_eq!(StrSpan::from("  text  ").trim().to_str(), "text");
    }

    #[test]
    fn trim_2() {
        assert_eq!(StrSpan::from("  text  text  ").trim().to_str(), "text  text");
    }

    #[test]
    fn trim_3() {
        assert_eq!(StrSpan::from("&#x20;text&#x20;").trim().to_str(), "text");
    }

    #[test]
    fn trim_4() {
        assert_eq!(StrSpan::from("&#x20;text&#x20;text&#x20;").trim().to_str(), "text&#x20;text");
    }

    #[test]
    fn do_not_trim_1() {
        assert_eq!(StrSpan::from("&#x40;text&#x50;").trim().to_str(), "&#x40;text&#x50;");
    }

    #[test]
    fn do_not_trim_2() {
        assert_eq!(StrSpan::from("&ref;text&apos;").trim().to_str(), "&ref;text&apos;");
    }
}
