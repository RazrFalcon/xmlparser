use std::char;
use std::str;
use std::ops::{Deref, DerefMut};

use {
    ByteStream,
    StreamError,
    StrSpan,
    XmlByteExt,
    XmlCharExt,
};

type Result<T> = ::std::result::Result<T, StreamError>;


/// Representation of the [Reference](https://www.w3.org/TR/xml/#NT-Reference) value.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Reference<'a> {
    /// An entity reference.
    ///
    /// <https://www.w3.org/TR/xml/#NT-EntityRef>
    Entity(&'a str),

    /// A character reference.
    ///
    /// <https://www.w3.org/TR/xml/#NT-CharRef>
    Char(char),
}


/// A streaming XML parsing interface.
#[derive(Clone, Copy, PartialEq)]
pub struct Stream<'a> {
    d: ByteStream<'a>,
}

impl<'a> Deref for Stream<'a> {
    type Target = ByteStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.d
    }
}

impl<'a> DerefMut for Stream<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.d
    }
}

impl<'a> From<ByteStream<'a>> for Stream<'a> {
    fn from(d: ByteStream<'a>) -> Self {
        Stream { d }
    }
}

impl<'a> From<&'a str> for Stream<'a> {
    fn from(text: &'a str) -> Self {
        ByteStream::new(text.into()).into()
    }
}

impl<'a> From<StrSpan<'a>> for Stream<'a> {
    fn from(span: StrSpan<'a>) -> Self {
        ByteStream::new(span).into()
    }
}

impl<'a> Stream<'a> {
    /// Skips whitespaces.
    ///
    /// Accepted values: `' ' \n \r \t &#x20; &#x9; &#xD; &#xA;`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmlparser::Stream;
    ///
    /// let mut s = Stream::from(" \t\n\r &#x20; ");
    /// s.skip_spaces();
    /// assert_eq!(s.at_end(), true);
    /// ```
    pub fn skip_spaces(&mut self) {
        while !self.at_end() {
            let c = self.curr_byte_unchecked();

            if c.is_xml_space() {
                self.advance(1);
            } else if c == b'&' {
                // Check for (#x20 | #x9 | #xD | #xA).
                let mut is_space = false;
                if let Some(Reference::Char(ch)) = self.try_consume_reference() {
                    if (ch as u32) < 255 && (ch as u8).is_xml_space() {
                        is_space = true;
                    }
                }

                if !is_space {
                    break;
                }
            } else {
                break;
            }
        }
    }

    /// Skips ASCII whitespaces.
    ///
    /// Accepted values: `' ' \n \r \t`.
    pub fn skip_ascii_spaces(&mut self) {
        while !self.at_end() && self.curr_byte_unchecked().is_xml_space() {
            self.advance(1);
        }
    }

    /// Checks if the stream is starts with a space.
    pub fn starts_with_space(&self) -> bool {
        if self.at_end() {
            return false;
        }

        let mut is_space = false;

        let c = self.curr_byte_unchecked();

        if c.is_xml_space() {
            is_space = true;
        } else if c == b'&' {
            // Check for (#x20 | #x9 | #xD | #xA).
            let mut s = self.clone();
            if let Ok(Reference::Char(v)) = s.consume_reference() {
                if (v as u32) < 255 && (v as u8).is_xml_space() {
                    is_space = true;
                }
            }
        }

        is_space
    }

    /// Consumes whitespaces.
    ///
    /// Like [`skip_spaces()`], but checks that first char is actually a space.
    ///
    /// [`skip_spaces()`]: #method.skip_spaces
    ///
    /// # Errors
    ///
    /// - `InvalidSpace`
    pub fn consume_spaces(&mut self) -> Result<()> {
        if self.at_end() {
            return Err(StreamError::UnexpectedEndOfStream);
        }

        if !self.starts_with_space() {
            let c = self.curr_byte_unchecked() as char;
            let pos = self.gen_text_pos();
            return Err(StreamError::InvalidSpace(c, pos));
        }

        self.skip_spaces();
        Ok(())
    }

    /// Consumes an XML character reference if there is one.
    ///
    /// On error will reset the position to the original.
    pub fn try_consume_reference(&mut self) -> Option<Reference<'a>> {
        let start = self.pos();

        // Consume reference on a substream.
        let mut s = self.clone();
        match s.consume_reference() {
            Ok(r) => {
                // If the current data is a reference than advance the current stream
                // by number of bytes read by substream.
                self.advance(s.pos() - start);
                Some(r)
            }
            Err(_) => {
                None
            }
        }
    }

    /// Consumes an XML reference.
    ///
    /// Consumes according to: <https://www.w3.org/TR/xml/#NT-Reference>
    ///
    /// # Errors
    ///
    /// - `InvalidReference`
    pub fn consume_reference(&mut self) -> Result<Reference<'a>> {
        self._consume_reference().map_err(|_| StreamError::InvalidReference)
    }

    fn _consume_reference(&mut self) -> Result<Reference<'a>> {
        if !self.try_consume_byte(b'&') {
            return Err(StreamError::InvalidReference);
        }

        let reference = if self.try_consume_byte(b'#') {
            let (value, radix) = if self.try_consume_byte(b'x') {
                let value = self.consume_bytes(|_, c| c.is_xml_hex_digit()).as_str();
                (value, 16)
            } else {
                let value = self.consume_bytes(|_, c| c.is_xml_digit()).as_str();
                (value, 10)
            };

            let n = u32::from_str_radix(value, radix).map_err(|_| StreamError::InvalidReference)?;

            let c = char::from_u32(n).unwrap_or('\u{FFFD}');
            if !c.is_xml_char() {
                return Err(StreamError::InvalidReference);
            }

            Reference::Char(c)
        } else {
            let name = self.consume_name()?;
            match name.as_str() {
                "quot" => Reference::Char('"'),
                "amp"  => Reference::Char('&'),
                "apos" => Reference::Char('\''),
                "lt"   => Reference::Char('<'),
                "gt"   => Reference::Char('>'),
                _ => Reference::Entity(name.as_str()),
            }
        };

        self.consume_byte(b';')?;

        Ok(reference)
    }

    /// Consumes an XML name and returns it.
    ///
    /// Consumes according to: <https://www.w3.org/TR/xml/#NT-Name>
    ///
    /// # Errors
    ///
    /// - `InvalidName` - if name is empty or starts with an invalid char
    /// - `UnexpectedEndOfStream`
    pub fn consume_name(&mut self) -> Result<StrSpan<'a>> {
        let start = self.pos();
        self.skip_name()?;

        let name = self.slice_back(start);
        if name.is_empty() {
            return Err(StreamError::InvalidName);
        }

        Ok(name)
    }

    /// Skips an XML name.
    ///
    /// The same as `consume_name()`, but does not return a consumed name.
    ///
    /// # Errors
    ///
    /// - `InvalidName` - if name is empty or starts with an invalid char
    pub fn skip_name(&mut self) -> Result<()> {
        let mut iter = self.chars();
        if let Some(c) = iter.next() {
            if c.is_xml_name_start() {
                self.advance(c.len_utf8());
            } else {
                return Err(StreamError::InvalidName);
            }
        }

        for c in iter {
            if c.is_xml_name() {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Consumes a qualified XML name and returns it.
    ///
    /// Consumes according to: <https://www.w3.org/TR/xml-names/#ns-qualnames>
    ///
    /// # Errors
    ///
    /// - `InvalidName` - if name is empty or starts with an invalid char
    pub fn consume_qname(&mut self) -> Result<(StrSpan<'a>, StrSpan<'a>)> {
        let start = self.pos();

        let mut splitter = None;
        for c in self.chars() {
            if c == ':' {
                splitter = Some(self.pos());
                self.advance(1);
            } else if c.is_xml_name() {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }

        let (prefix, local) = if let Some(splitter) = splitter {
            let prefix = self.span().slice_region(start, splitter);
            let local = self.slice_back(splitter + 1);
            (prefix, local)
        } else {
            let local = self.slice_back(start);
            ("".into(), local)
        };

        if local.is_empty() {
            return Err(StreamError::InvalidName);
        }

        Ok((prefix, local))
    }

    /// Consumes `=`.
    ///
    /// Consumes according to: <https://www.w3.org/TR/xml/#NT-Eq>
    ///
    /// # Errors
    ///
    /// - `InvalidChar`
    /// - `UnexpectedEndOfStream`
    pub fn consume_eq(&mut self) -> Result<()> {
        self.skip_ascii_spaces();
        self.consume_byte(b'=')?;
        self.skip_ascii_spaces();

        Ok(())
    }

    /// Consumes quote.
    ///
    /// Consumes `'` or `"` and returns it.
    ///
    /// # Errors
    ///
    /// - `InvalidQuote`
    /// - `UnexpectedEndOfStream`
    pub fn consume_quote(&mut self) -> Result<u8> {
        let c = self.curr_byte()?;
        if c == b'\'' || c == b'"' {
            self.advance(1);
            Ok(c)
        } else {
            Err(StreamError::InvalidQuote(c as char, self.gen_text_pos()))
        }
    }
}
