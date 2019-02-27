use std::char;
use std::str;
use std::cmp;

use {
    StreamError,
    StrSpan,
    TextPos,
};

type Result<T> = ::std::result::Result<T, StreamError>;


/// A streaming text parsing interface.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ByteStream<'a> {
    pos: usize,
    end: usize,
    span: StrSpan<'a>,
}

impl<'a> ByteStream<'a> {
    pub(crate) fn new(span: StrSpan<'a>) -> Self {
        ByteStream {
            pos: 0,
            end: span.as_str().len(),
            span,
        }
    }

    /// Returns an underling string span.
    #[inline]
    pub fn span(&self) -> StrSpan<'a> {
        self.span
    }

    /// Returns current position.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets current position equal to the end.
    ///
    /// Used to indicate end of parsing on error.
    #[inline]
    pub fn jump_to_end(&mut self) {
        self.pos = self.end;
    }

    /// Checks if the stream is reached the end.
    ///
    /// Any [`pos()`] value larger than original text length indicates stream end.
    ///
    /// Accessing stream after reaching end via safe methods will produce
    /// an `UnexpectedEndOfStream` error.
    ///
    /// Accessing stream after reaching end via *_unchecked methods will produce
    /// a Rust's bound checking error.
    ///
    /// [`pos()`]: #method.pos
    #[inline]
    pub fn at_end(&self) -> bool {
        self.pos >= self.end
    }

    /// Returns a byte from a current stream position.
    ///
    /// # Errors
    ///
    /// - `UnexpectedEndOfStream`
    #[inline]
    pub fn curr_byte(&self) -> Result<u8> {
        if self.at_end() {
            return Err(StreamError::UnexpectedEndOfStream);
        }

        Ok(self.curr_byte_unchecked())
    }

    /// Returns a byte from a current stream position.
    ///
    /// # Panics
    ///
    /// - if the current position is after the end of the data
    #[inline]
    pub fn curr_byte_unchecked(&self) -> u8 {
        self.span.as_bytes()[self.pos]
    }

    /// Returns a next byte from a current stream position.
    ///
    /// # Errors
    ///
    /// - `UnexpectedEndOfStream`
    #[inline]
    pub fn next_byte(&self) -> Result<u8> {
        if self.pos + 1 >= self.end {
            return Err(StreamError::UnexpectedEndOfStream);
        }

        Ok(self.span.as_bytes()[self.pos + 1])
    }

    /// Advances by `n` bytes.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// use xmlparser::Stream;
    ///
    /// let mut s = Stream::from("text");
    /// s.advance(2); // ok
    /// s.advance(20); // will cause a panic via debug_assert!().
    /// ```
    #[inline]
    pub fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.end);
        self.pos += n;
    }

    /// Checks that the stream starts with a selected text.
    ///
    /// We are using `&[u8]` instead of `&str` for performance reasons.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmlparser::Stream;
    ///
    /// let mut s = Stream::from("Some text.");
    /// s.advance(5);
    /// assert_eq!(s.starts_with(b"text"), true);
    /// assert_eq!(s.starts_with(b"long"), false);
    /// ```
    #[inline]
    pub fn starts_with(&self, text: &[u8]) -> bool {
        self.span.as_bytes()[self.pos..self.end].starts_with(text)
    }

    /// Consumes the current byte if it's equal to the provided byte.
    ///
    /// # Errors
    ///
    /// - `InvalidChar`
    /// - `UnexpectedEndOfStream`
    ///
    /// # Examples
    ///
    /// ```
    /// use xmlparser::Stream;
    ///
    /// let mut s = Stream::from("Some text.");
    /// assert!(s.consume_byte(b'S').is_ok());
    /// assert!(s.consume_byte(b'o').is_ok());
    /// assert!(s.consume_byte(b'm').is_ok());
    /// assert!(s.consume_byte(b'q').is_err());
    /// ```
    pub fn consume_byte(&mut self, c: u8) -> Result<()> {
        if self.curr_byte()? != c {
            return Err(
                StreamError::InvalidChar(
                    vec![self.curr_byte_unchecked(), c],
                    self.gen_text_pos(),
                )
            );
        }

        self.advance(1);
        Ok(())
    }

    /// Tries to consume the current byte if it's equal to the provided byte.
    ///
    /// Unlike `consume_byte()` will not return any errors.
    pub fn try_consume_byte(&mut self, c: u8) -> bool {
        match self.curr_byte() {
            Ok(b) if b == c => {
                self.advance(1);
                true
            }
            _ => false,
        }
    }

    /// Skips selected string.
    ///
    /// # Errors
    ///
    /// - `InvalidString`
    pub fn skip_string(&mut self, text: &[u8]) -> Result<()> {
        if !self.starts_with(text) {
            let len = cmp::min(text.len(), self.end - self.pos);
            // Collect chars and do not slice a string,
            // because the `len` can be on the char boundary.
            // Which lead to a panic.
            let actual = self.span.as_str()[self.pos..].chars().take(len).collect();

            // Assume that all input `text` are valid UTF-8 strings, so unwrap is safe.
            let expected = str::from_utf8(text).unwrap().to_owned();

            let pos = self.gen_text_pos();

            return Err(StreamError::InvalidString(vec![actual, expected], pos));
        }

        self.advance(text.len());
        Ok(())
    }

    /// Consumes bytes by the predicate and returns them.
    ///
    /// The result can be empty.
    pub fn consume_bytes<F>(&mut self, f: F) -> StrSpan<'a>
        where F: Fn(&ByteStream, u8) -> bool
    {
        let start = self.pos;
        self.skip_bytes(f);
        self.slice_back(start)
    }

    /// Skips bytes by the predicate.
    pub fn skip_bytes<F>(&mut self, f: F)
        where F: Fn(&ByteStream, u8) -> bool
    {
        while !self.at_end() && f(self, self.curr_byte_unchecked()) {
            self.advance(1);
        }
    }

    /// Consumes chars by the predicate and returns them.
    ///
    /// The result can be empty.
    pub fn consume_chars<F>(&mut self, f: F) -> StrSpan<'a>
        where F: Fn(&ByteStream, char) -> bool
    {
        let start = self.pos;
        self.skip_chars(f);
        self.slice_back(start)
    }

    /// Skips chars by the predicate.
    pub fn skip_chars<F>(&mut self, f: F)
        where F: Fn(&ByteStream, char) -> bool
    {
        for c in self.chars() {
            if f(self, c) {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }
    }

    #[inline]
    pub(crate) fn chars(&self) -> str::Chars<'a> {
        self.span.as_str()[self.pos..self.end].chars()
    }

    /// Slices data from `pos` to the current position.
    #[inline]
    pub fn slice_back(&self, pos: usize) -> StrSpan<'a> {
        self.span.slice_region(pos, self.pos)
    }

    /// Slices data from the current position to the end.
    #[inline]
    pub fn slice_tail(&self) -> StrSpan<'a> {
        self.span.slice_region(self.pos, self.end)
    }

    /// Calculates a current absolute position.
    ///
    /// This operation is very expensive. Use only for errors.
    #[inline(never)]
    pub fn gen_text_pos(&self) -> TextPos {
        let text = self.span.full_str();
        let end = self.pos + self.span.start();

        let row = Self::calc_curr_row(text, end);
        let col = Self::calc_curr_col(text, end);
        TextPos::new(row, col)
    }

    /// Calculates an absolute position at `pos`.
    ///
    /// This operation is very expensive. Use only for errors.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = xmlparser::Stream::from("text");
    ///
    /// assert_eq!(s.gen_text_pos_from(2), xmlparser::TextPos::new(1, 3));
    /// assert_eq!(s.gen_text_pos_from(9999), xmlparser::TextPos::new(1, 5));
    /// ```
    #[inline(never)]
    pub fn gen_text_pos_from(&self, pos: usize) -> TextPos {
        let mut s = self.clone();
        s.pos = cmp::min(pos, s.span.full_str().len());
        s.gen_text_pos()
    }

    fn calc_curr_row(text: &str, end: usize) -> u32 {
        let mut row = 1;
        for c in &text.as_bytes()[..end] {
            if *c == b'\n' {
                row += 1;
            }
        }

        row
    }

    fn calc_curr_col(text: &str, end: usize) -> u32 {
        let mut col = 1;
        for c in text[..end].chars().rev() {
            if c == '\n' {
                break;
            } else {
                col += 1;
            }
        }

        col
    }
}
