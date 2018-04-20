use std::str;
use std::io::Write;

use {
    Stream,
    StrSpan,
};


const BUF_END: usize = 4;

/// Spaces processing type.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum XmlSpace {
    Default,
    Preserve,
}


/// XML escaped text to plain text converter.
///
/// Processing is done as described in: <https://www.w3.org/TR/SVG11/text.html#WhiteSpace>
///
/// # Examples
///
/// Allocation free version:
///
/// ```
/// use std::str;
/// use xmlparser::TextUnescape;
///
/// let v: Vec<_> = TextUnescape::from("&gt;").collect();
/// let s = str::from_utf8(&v).unwrap();
/// assert_eq!(s, ">");
/// ```
///
/// Version which will allocate a `String`:
///
/// ```
/// use xmlparser::{TextUnescape, XmlSpace};
///
/// let s = TextUnescape::unescape("&gt;", XmlSpace::Default);
/// assert_eq!(s, ">");
/// ```
pub struct TextUnescape<'a> {
    stream: Stream<'a>,
    buf: [u8; BUF_END],
    buf_idx: usize,
    preserve_spaces: bool,
    prev: u8,
}

impl<'a> From<&'a str> for TextUnescape<'a> {
    fn from(text: &'a str) -> Self {
        Self::from(StrSpan::from(text))
    }
}

impl<'a> From<StrSpan<'a>> for TextUnescape<'a> {
    fn from(span: StrSpan<'a>) -> Self {
        TextUnescape {
            stream: Stream::from(span),
            buf: [0xFF; BUF_END],
            buf_idx: BUF_END,
            preserve_spaces: false,
            prev: 0,
        }
    }
}

impl<'a> TextUnescape<'a> {
    /// Converts provided text into an unescaped one.
    pub fn unescape(text: &str, space: XmlSpace) -> String {
        let mut v = Vec::new();
        let mut t = TextUnescape::from(text);
        t.set_xml_space(space);
        for c in t {
            v.push(c);
        }

        str::from_utf8(&v).unwrap().to_owned()
    }

    /// Sets the flag that prevents spaces from being striped.
    pub fn set_xml_space(&mut self, kind: XmlSpace) {
        self.preserve_spaces = kind == XmlSpace::Preserve;
    }
}

impl<'a> Iterator for TextUnescape<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf_idx != BUF_END {
            let c = self.buf[self.buf_idx];

            if c != 0xFF {
                self.buf_idx += 1;
                return Some(c);
            } else {
                self.buf_idx = BUF_END;
            }
        }

        if self.stream.at_end() {
            return None;
        }

        let mut c = self.stream.curr_byte().unwrap();

        // Check for XML character entity references.
        if c == b'&' {
            if let Some(ch) = self.stream.try_consume_char_reference() {
                self.buf = [0xFF; 4];

                write!(&mut self.buf[..], "{}", ch).unwrap();

                c = self.buf[0];
                self.buf_idx = 1;
            } else {
                self.stream.advance(1);
            }
        } else {
            self.stream.advance(1);
        }

        // \n and \t should be converted into spaces.
        c = match c {
            b'\n' | b'\t' => b' ',
            _ => c,
        };

        // \r should be ignored.
        if c == b'\r' {
            return self.next();
        }

        // Skip continuous spaces when `preserve_spaces` is not set.
        if !self.preserve_spaces && c == b' ' && c == self.prev {
            return self.next();
        }

        self.prev = c;

        Some(c)
    }
}
