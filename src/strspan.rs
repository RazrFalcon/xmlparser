use std::fmt;
use std::ops::Range;


/// An immutable string slice.
///
/// Unlike `&str` contains a reference to the original string
/// and a span region.
#[must_use]
#[derive(Clone, Copy, PartialEq)]
pub struct StrSpan<'a> {
    text: &'a str,
    span: &'a str,
    start: usize,
}

impl<'a> From<&'a str> for StrSpan<'a> {
    fn from(text: &'a str) -> Self {
        StrSpan {
            text,
            start: 0,
            span: text,
        }
    }
}

impl<'a> StrSpan<'a> {
    /// Constructs a new `StrSpan` from substring.
    #[inline]
    pub(crate) fn from_substr(text: &str, start: usize, end: usize) -> StrSpan {
        debug_assert!(start <= end);
        StrSpan { text, span: &text[start..end], start }
    }

    /// Returns a start position of the span.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns a end position of the span.
    #[inline]
    pub fn end(&self) -> usize {
        self.start + self.span.len()
    }

    /// Returns a end position of the span.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.start..self.end()
    }

    /// Returns a length of the span.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// Returns a span slice.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        &self.span
    }

    /// Returns a span slice as bytes.
    #[inline]
    pub(crate) fn as_bytes(&self) -> &'a [u8] {
        self.span.as_bytes()
    }

    /// Returns an underling string.
    #[inline]
    pub fn full_str(&self) -> &'a str {
        self.text
    }

    /// Returns an underling string region as `StrSpan`.
    #[inline]
    pub(crate) fn slice_region(&self, start: usize, end: usize) -> StrSpan<'a> {
        let start = self.start + start;
        let end = self.start + end;

        StrSpan::from_substr(self.text, start, end)
    }
}

impl<'a> fmt::Debug for StrSpan<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StrSpan({:?} {}..{})", self.as_str(), self.start(), self.end())
    }
}

impl<'a> fmt::Display for StrSpan<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
