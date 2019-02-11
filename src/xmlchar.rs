/// Extension methods for XML-subset only operations.
pub trait XmlCharExt {
    /// Checks if the value is within the
    /// [NameStartChar](https://www.w3.org/TR/xml/#NT-NameStartChar) range.
    fn is_xml_name_start(&self) -> bool;

    /// Checks if the value is within the
    /// [NameChar](https://www.w3.org/TR/xml/#NT-NameChar) range.
    fn is_xml_name(&self) -> bool;

    /// Checks if the value is within the
    /// [Char](https://www.w3.org/TR/xml/#NT-Char) range.
    fn is_xml_char(&self) -> bool;
}

impl XmlCharExt for char {
    #[inline]
    fn is_xml_name_start(&self) -> bool {
        match *self {
            'A'...'Z' | 'a'...'z' | ':' | '_' => true, // shortcut
            _ => {
                match *self as u32 {
                      0x0000C0...0x0000D6
                    | 0x0000D8...0x0000F6
                    | 0x0000F8...0x0002FF
                    | 0x000370...0x00037D
                    | 0x00037F...0x001FFF
                    | 0x00200C...0x00200D
                    | 0x002070...0x00218F
                    | 0x002C00...0x002FEF
                    | 0x003001...0x00D7FF
                    | 0x00F900...0x00FDCF
                    | 0x00FDF0...0x00FFFD
                    | 0x010000...0x0EFFFF => true,
                    _ => false,
                }
            }
        }
    }

    #[inline]
    fn is_xml_name(&self) -> bool {
        if self.is_xml_name_start() {
            return true;
        }

        match *self as u32 {
              0x002D // -
            | 0x002E // .
            | 0x00B7
            | 0x0030...0x0039 // 0...9
            | 0x0300...0x036F
            | 0x203F...0x2040 => true,
            _ => false,
        }
    }

    #[inline]
    fn is_xml_char(&self) -> bool {
        match *self as u32 {
              0x000009
            | 0x00000A
            | 0x00000D
            | 0x000020...0x000D7FF
            | 0x00E000...0x000FFFD
            | 0x010000...0x010FFFF => true,
            _ => false,
        }
    }
}


/// Extension methods for XML-subset only operations.
pub trait XmlByteExt {
    /// Checks if a byte is a digit.
    ///
    /// `[0-9]`
    fn is_xml_digit(&self) -> bool;

    /// Checks if a byte is a hex digit.
    ///
    /// `[0-9A-Fa-f]`
    fn is_xml_hex_digit(&self) -> bool;

    /// Checks if a byte is a space.
    ///
    /// `[ \r\n\t]`
    fn is_xml_space(&self) -> bool;

    /// Checks if a byte is an ASCII char.
    ///
    /// `[A-Za-z]`
    fn is_xml_letter(&self) -> bool;
}

impl XmlByteExt for u8 {
    #[inline]
    fn is_xml_digit(&self) -> bool {
        matches!(*self, b'0'...b'9')
    }

    #[inline]
    fn is_xml_hex_digit(&self) -> bool {
        matches!(*self, b'0'...b'9' | b'A'...b'F' | b'a'...b'f')
    }

    #[inline]
    fn is_xml_space(&self) -> bool {
        matches!(*self, b' ' | b'\t' | b'\n' | b'\r')
    }

    #[inline]
    fn is_xml_letter(&self) -> bool {
        matches!(*self, b'A'...b'Z' | b'a'...b'z')
    }
}
