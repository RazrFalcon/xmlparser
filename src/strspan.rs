use std::fmt;
use std::ops::Range;

use Stream;


/// An immutable string slice.
///
/// Unlike `&str` contains a reference to the original string
/// and a span region.
#[must_use]
#[derive(Clone, Copy, PartialEq)]
pub struct StrSpan<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

impl<'a> From<&'a str> for StrSpan<'a> {
    fn from(text: &'a str) -> Self {
        StrSpan {
            text,
            start: 0,
            end: text.len(),
        }
    }
}

impl<'a> StrSpan<'a> {
    /// Constructs a new `StrSpan` from substring.
    #[inline]
    pub fn from_substr(text: &str, start: usize, end: usize) -> StrSpan {
        debug_assert!(start <= end);
        debug_assert!(text.is_char_boundary(start));
        debug_assert!(text.is_char_boundary(end));

        StrSpan { text, start, end }
    }

    /// Returns a start position of the span.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns a end position of the span.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns a end position of the span.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns a length of the span.
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns a length of the span.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a length of the span underling string.
    #[inline]
    pub fn full_len(&self) -> usize {
        self.text.len()
    }

    /// Returns a span slice.
    ///
    /// A bit expensive, since Rust checks for char boundary.
    ///
    /// # Panics
    ///
    /// - if the span is out of bounds of the original string
    /// - if the start or end positions is not on a char boundary
    #[inline]
    pub fn to_str(&self) -> &'a str {
        &self.text[self.start..self.end]
    }

    /// Returns a span slice as bytes.
    ///
    /// The same as `to_str` but does not involve char boundary checking.
    ///
    /// # Panics
    ///
    /// - if the span is out of bounds of the original string
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        &self.text.as_bytes()[self.start..self.end]
    }

    /// Returns an underling string region as `StrSpan`.
    #[inline]
    pub fn slice_region(&self, start: usize, end: usize) -> StrSpan<'a> {
        let start = self.start + start;
        let end = self.start + end;

        StrSpan::from_substr(self.text, start, end)
    }

    /// Returns an underling string.
    #[inline]
    pub fn full_str(&self) -> &'a str {
        self.text
    }

    /// Returns a trimmed version of this `StrSpan`.
    ///
    /// Removes only leading and trailing spaces.
    ///
    /// This function will trim escaped spaces (aka `&#x20;`) too.
    pub fn trim(&self) -> StrSpan<'a> {
        let mut s = Stream::from(*self);
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
