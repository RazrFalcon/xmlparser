/*!

*xmlparser* is a low-level, pull-based, zero-allocation
[XML 1.0](https://www.w3.org/TR/xml/) parser.

## Example

```rust
for token in xmlparser::Tokenizer::from("<tagname name='value'/>") {
    println!("{:?}", token);
}
```

## Why a new library

This library is basically a low-level XML tokenizer that preserves a position of the tokens
and does not intend to be used directly.
If you are looking for a more high-level solution - checkout
[roxmltree](https://github.com/RazrFalcon/roxmltree).

## Benefits

- All tokens contain `StrSpan` objects which contain a position of the data in the original document.
- Good error processing. All error types contain position (line:column) where it occurred.
- No heap allocations.
- No dependencies.
- Tiny. ~1500 LOC and ~40KiB in the release build according to the `cargo-bloat`.

## Limitations

- Currently, only ENTITY objects are parsed from the DOCTYPE. Other ignored.
- No tree structure validation. So an XML like `<root><child></root></child>`
  or a string without root element
  will be parsed without errors. You should check for this manually.
  On the other hand `<a/><a/>` will lead to an error.
- Duplicated attributes is not an error. So an XML like `<item a="v1" a="v2"/>`
  will be parsed without errors. You should check for this manually.
- UTF-8 only.

## Safety

- The library must not panic. Any panic considered as a critical bug
  and should be reported.
- The library forbids the unsafe code.
*/

#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

#![doc(html_root_url = "https://docs.rs/xmlparser/0.11.0")]

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(ellipsis_inclusive_range_patterns)]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;


use core::fmt;


macro_rules! matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => true,
            _ => false
        }
    }
}


mod error;
mod stream;
mod strspan;
mod xmlchar;

pub use error::*;
pub use stream::*;
pub use strspan::*;
pub use xmlchar::*;


/// An XML token.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Token<'a> {
    /// Declaration token.
    ///
    /// ```text
    /// <?xml version='1.0' encoding='UTF-8' standalone='yes'?>
    ///                ---                                      - version
    ///                               -----                     - encoding?
    ///                                                  ---    - standalone?
    /// ------------------------------------------------------- - span
    /// ```
    Declaration {
        version: StrSpan<'a>,
        encoding: Option<StrSpan<'a>>,
        standalone: Option<bool>,
        span: StrSpan<'a>,
    },

    /// Processing instruction token.
    ///
    /// ```text
    /// <?target content?>
    ///   ------           - target
    ///          -------   - content?
    /// ------------------ - span
    /// ```
    ProcessingInstruction {
        target: StrSpan<'a>,
        content: Option<StrSpan<'a>>,
        span: StrSpan<'a>,
    },

    /// Comment token.
    ///
    /// ```text
    /// <!-- text -->
    ///     ------    - text
    /// ------------- - span
    /// ```
    Comment {
        text: StrSpan<'a>,
        span: StrSpan<'a>,
    },

    /// DOCTYPE start token.
    ///
    /// ```text
    /// <!DOCTYPE greeting SYSTEM "hello.dtd" [
    ///           --------                      - name
    ///                    ------------------   - external_id?
    /// --------------------------------------- - span
    /// ```
    DtdStart {
        name: StrSpan<'a>,
        external_id: Option<ExternalId<'a>>,
        span: StrSpan<'a>,
    },

    /// Empty DOCTYPE token.
    ///
    /// ```text
    /// <!DOCTYPE greeting SYSTEM "hello.dtd">
    ///           --------                     - name
    ///                    ------------------  - external_id?
    /// -------------------------------------- - span
    /// ```
    EmptyDtd {
        name: StrSpan<'a>,
        external_id: Option<ExternalId<'a>>,
        span: StrSpan<'a>,
    },

    /// ENTITY token.
    ///
    /// Can appear only inside the DTD.
    ///
    /// ```text
    /// <!ENTITY ns_extend "http://test.com">
    ///          ---------                    - name
    ///                     ---------------   - definition
    /// ------------------------------------- - span
    /// ```
    EntityDeclaration {
        name: StrSpan<'a>,
        definition: EntityDefinition<'a>,
        span: StrSpan<'a>,
    },

    /// DOCTYPE end token.
    ///
    /// ```text
    /// <!DOCTYPE svg [
    ///    ...
    /// ]>
    /// -- - span
    /// ```
    DtdEnd {
        span: StrSpan<'a>,
    },

    /// Element start token.
    ///
    /// ```text
    /// <ns:elem attr="value"/>
    ///  --                     - prefix
    ///     ----                - local
    /// --------                - span
    /// ```
    ElementStart {
        prefix: StrSpan<'a>,
        local: StrSpan<'a>,
        span: StrSpan<'a>,
    },

    /// Attribute token.
    ///
    /// ```text
    /// <elem ns:attr="value"/>
    ///       --              - prefix
    ///          ----         - local
    ///                -----  - value
    ///       --------------- - span
    /// ```
    Attribute {
        prefix: StrSpan<'a>,
        local: StrSpan<'a>,
        value: StrSpan<'a>,
        span: StrSpan<'a>,
    },

    /// Element end token.
    ///
    /// ```text
    /// <ns:elem>text</ns:elem>
    ///                         - ElementEnd::Open
    ///         -               - span
    /// ```
    ///
    /// ```text
    /// <ns:elem>text</ns:elem>
    ///                -- ----  - ElementEnd::Close(prefix, local)
    ///              ---------- - span
    /// ```
    ///
    /// ```text
    /// <ns:elem/>
    ///                         - ElementEnd::Empty
    ///         --              - span
    /// ```
    ElementEnd {
        end: ElementEnd<'a>,
        span: StrSpan<'a>,
    },

    /// Text token.
    ///
    /// Contains text between elements including whitespaces.
    /// Basically everything between `>` and `<`.
    /// Except `]]>`, which is not allowed and will lead to an error.
    ///
    /// ```text
    /// <p> text </p>
    ///    ------     - text
    /// ```
    ///
    /// The token span is equal to the `text`.
    Text {
        text: StrSpan<'a>,
    },

    /// CDATA token.
    ///
    /// ```text
    /// <p><![CDATA[text]]></p>
    ///             ----        - text
    ///    ----------------     - span
    /// ```
    Cdata {
        text: StrSpan<'a>,
        span: StrSpan<'a>,
    },
}


/// `ElementEnd` token.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ElementEnd<'a> {
    /// Indicates `>`
    Open,
    /// Indicates `</name>`
    Close(StrSpan<'a>, StrSpan<'a>),
    /// Indicates `/>`
    Empty,
}


/// Representation of the [ExternalID](https://www.w3.org/TR/xml/#NT-ExternalID) value.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ExternalId<'a> {
    System(StrSpan<'a>),
    Public(StrSpan<'a>, StrSpan<'a>),
}


/// Representation of the [EntityDef](https://www.w3.org/TR/xml/#NT-EntityDef) value.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EntityDefinition<'a> {
    EntityValue(StrSpan<'a>),
    ExternalId(ExternalId<'a>),
}


type Result<T> = core::result::Result<T, Error>;
type StreamResult<T> = core::result::Result<T, StreamError>;


/// List of token types.
///
/// For internal use and errors.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum TokenType {
    XMLDecl,
    Comment,
    PI,
    DoctypeDecl,
    ElementDecl,
    AttlistDecl,
    EntityDecl,
    NotationDecl,
    DoctypeEnd,
    ElementStart,
    ElementClose,
    Attribute,
    CDSect,
    Whitespace,
    CharData,
    Unknown,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            TokenType::XMLDecl => "Declaration",
            TokenType::Comment => "Comment",
            TokenType::PI => "Processing Instruction",
            TokenType::DoctypeDecl => "Doctype Declaration",
            TokenType::ElementDecl => "Doctype Element Declaration",
            TokenType::AttlistDecl => "Doctype Attributes Declaration",
            TokenType::EntityDecl => "Doctype Entity Declaration",
            TokenType::NotationDecl => "Doctype Notation Declaration",
            TokenType::DoctypeEnd => "Doctype End",
            TokenType::ElementStart => "Element Start",
            TokenType::ElementClose => "Element Close",
            TokenType::Attribute => "Attribute",
            TokenType::CDSect => "CDATA",
            TokenType::Whitespace => "Whitespace",
            TokenType::CharData => "Character Data",
            TokenType::Unknown => "Unknown",
        };

        write!(f, "{}", s)
    }
}


#[derive(Clone, Copy, PartialEq)]
enum State {
    Start,
    Dtd,
    AfterDtd,
    Elements,
    Attributes,
    AfterElements,
    End,
}


/// Tokenizer for the XML structure.
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    state: State,
    depth: usize,
    fragment_parsing: bool,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    #[inline]
    fn from(text: &'a str) -> Self {
        Self::from(StrSpan::from(text))
    }
}

impl<'a> From<StrSpan<'a>> for Tokenizer<'a> {
    #[inline]
    fn from(span: StrSpan<'a>) -> Self {
        Tokenizer {
            stream: Stream::from(span),
            state: State::Start,
            depth: 0,
            fragment_parsing: false,
        }
    }
}


/// Shorthand for:
///
/// ```no_run
/// let start = stream.pos() - 2; // or any other number
/// some_func().map_err(|e|
///     Error::InvalidToken(Token::SomeToken, stream.gen_error_pos_from(start), Some(e))
/// )
/// ```
macro_rules! map_err_at {
    ($fun:expr, $token:expr, $stream:expr, $d:expr) => {{
        let mut start = $stream.pos() as isize + $d;
        debug_assert!(start >= 0);
        if start < 0 { start = 0; }
        $fun.map_err(|e|
            Error::InvalidToken($token, $stream.gen_text_pos_from(start as usize), Some(e))
        )
    }}
}

impl<'a> Tokenizer<'a> {
    /// Enables document fragment parsing.
    ///
    /// By default, `xmlparser` will check for DTD, root element, etc.
    /// But if we have to parse an XML fragment, it will lead to an error.
    /// This method switches the parser to the root element content parsing mode.
    /// So it will treat any data as a content of the root element.
    pub fn enable_fragment_mode(&mut self) {
        self.state = State::Elements;
        self.fragment_parsing = true;
    }

    fn parse_next_impl(s: &mut Stream<'a>, state: State) -> Option<Result<Token<'a>>> {
        if s.at_end() {
            return None;
        }

        let start = s.pos();

        if start == 0 {
            // Skip UTF-8 BOM.
            if s.starts_with(&[0xEF, 0xBB, 0xBF]) {
                s.advance(3);
            }
        }

        macro_rules! parse_token_type {
            () => ({
                match Self::parse_token_type(s, state) {
                    Ok(v) => v,
                    Err(_) => {
                        let pos = s.gen_text_pos_from(start);
                        return Some(Err(Error::UnknownToken(pos)));
                    }
                }
            })
        }

        macro_rules! gen_err {
            ($token_type:expr) => ({
                let pos = s.gen_text_pos_from(start);
                if $token_type == TokenType::Unknown {
                    return Some(Err(Error::UnknownToken(pos)));
                } else {
                    return Some(Err(Error::UnexpectedToken($token_type, pos)));
                }
            })
        }

        let t = match state {
            State::Start => {
                let token_type = parse_token_type!();
                match token_type {
                    TokenType::XMLDecl => {
                        // XML declaration allowed only at the start of the document.
                        if start == 0 {
                            Self::parse_declaration(s)
                        } else {
                            gen_err!(token_type);
                        }
                    }
                    TokenType::Comment => {
                        Self::parse_comment(s)
                    }
                    TokenType::PI => {
                        Self::parse_pi(s)
                    }
                    TokenType::DoctypeDecl => {
                        Self::parse_doctype(s)
                    }
                    TokenType::ElementStart => {
                        Self::parse_element_start(s)
                    }
                    TokenType::Whitespace => {
                        s.skip_spaces();
                        return Self::parse_next_impl(s, state);
                    }
                    _ => {
                        gen_err!(token_type);
                    }
                }
            }
            State::Dtd => {
                let token_type = parse_token_type!();
                match token_type {
                      TokenType::ElementDecl
                    | TokenType::NotationDecl
                    | TokenType::AttlistDecl => {
                        if Self::consume_decl(s).is_err() {
                            gen_err!(token_type);
                        }

                        return Self::parse_next_impl(s, state);
                    }
                    TokenType::EntityDecl => {
                        Self::parse_entity_decl(s)
                    }
                    TokenType::Comment => {
                        Self::parse_comment(s)
                    }
                    TokenType::PI => {
                        Self::parse_pi(s)
                    }
                    TokenType::DoctypeEnd => {
                        Ok(Token::DtdEnd { span: s.slice_back(s.pos() - 2) })
                    }
                    TokenType::Whitespace => {
                        s.skip_spaces();
                        return Self::parse_next_impl(s, state);
                    }
                    _ => {
                        gen_err!(token_type);
                    }
                }
            }
            State::AfterDtd => {
                let token_type = parse_token_type!();
                match token_type {
                    TokenType::Comment => {
                        Self::parse_comment(s)
                    }
                    TokenType::PI => {
                        Self::parse_pi(s)
                    }
                    TokenType::ElementStart => {
                        Self::parse_element_start(s)
                    }
                    TokenType::Whitespace => {
                        s.skip_spaces();
                        return Self::parse_next_impl(s, state);
                    }
                    _ => {
                        gen_err!(token_type);
                    }
                }
            }
            State::Elements => {
                let token_type = parse_token_type!();
                match token_type {
                    TokenType::ElementStart => {
                        Self::parse_element_start(s)
                    }
                    TokenType::ElementClose => {
                        Self::parse_close_element(s)
                    }
                    TokenType::CDSect => {
                        Self::parse_cdata(s)
                    }
                    TokenType::PI => {
                        Self::parse_pi(s)
                    }
                    TokenType::Comment => {
                        Self::parse_comment(s)
                    }
                    TokenType::CharData => {
                        Self::parse_text(s)
                    }
                    _ => {
                        gen_err!(token_type);
                    }
                }
            }
            State::Attributes => {
                Self::parse_attribute(s).map_err(|e|
                    Error::InvalidToken(TokenType::Attribute,
                                        s.gen_text_pos_from(start), Some(e)))
            }
            State::AfterElements => {
                let token_type = parse_token_type!();
                match token_type {
                    TokenType::Comment => {
                        Self::parse_comment(s)
                    }
                    TokenType::PI => {
                        Self::parse_pi(s)
                    }
                    TokenType::Whitespace => {
                        s.skip_spaces();
                        return Self::parse_next_impl(s, state);
                    }
                    _ => {
                        gen_err!(token_type);
                    }
                }
            }
            State::End => {
                return None;
            }
        };

        Some(t)
    }

    fn parse_token_type(s: &mut Stream, state: State) -> StreamResult<TokenType> {
        let c1 = s.curr_byte()?;

        let t = match c1 {
            b'<' => {
                s.advance(1);

                let c2 = s.curr_byte()?;
                match c2 {
                    b'?' => {
                        // TODO: technically, we should check for any whitespace
                        if s.starts_with(b"?xml ") {
                            s.advance(5);
                            TokenType::XMLDecl
                        } else {
                            s.advance(1);
                            TokenType::PI
                        }
                    }
                    b'!' => {
                        s.advance(1);

                        let c3 = s.curr_byte()?;
                        match c3 {
                            b'-' if s.starts_with(b"--") => {
                                s.advance(2);
                                TokenType::Comment
                            }
                            b'D' if s.starts_with(b"DOCTYPE") => {
                                s.advance(7);
                                TokenType::DoctypeDecl
                            }
                            b'E' if s.starts_with(b"ELEMENT") => {
                                s.advance(7);
                                TokenType::ElementDecl
                            }
                            b'A' if s.starts_with(b"ATTLIST") => {
                                s.advance(7);
                                TokenType::AttlistDecl
                            }
                            b'E' if s.starts_with(b"ENTITY") => {
                                s.advance(6);
                                TokenType::EntityDecl
                            }
                            b'N' if s.starts_with(b"NOTATION") => {
                                s.advance(8);
                                TokenType::NotationDecl
                            }
                            b'[' if s.starts_with(b"[CDATA[") => {
                                s.advance(7);
                                TokenType::CDSect
                            }
                            _ => {
                                TokenType::Unknown
                            }
                        }
                    }
                    b'/' => {
                        s.advance(1);
                        TokenType::ElementClose
                    }
                    _ => {
                        TokenType::ElementStart
                    }
                }
            }
            b']' if state == State::Dtd && s.starts_with(b"]>") => {
                s.advance(2);
                TokenType::DoctypeEnd
            }
            _ => {
                match state {
                    State::Start | State::AfterDtd | State::AfterElements | State::Dtd => {
                        if s.starts_with_space() {
                            TokenType::Whitespace
                        } else {
                            TokenType::Unknown
                        }
                    }
                    State::Elements => {
                        TokenType::CharData
                    }
                    _ => {
                        TokenType::Unknown
                    }
                }
            }
        };

        Ok(t)
    }

    fn parse_declaration(s: &mut Stream<'a>) -> Result<Token<'a>> {
        map_err_at!(Self::parse_declaration_impl(s), TokenType::XMLDecl, s, -6)
    }

    // XMLDecl ::= '<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>'
    fn parse_declaration_impl(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        let start = s.pos() - 6;

        let version = Self::parse_version_info(s)?;
        let encoding = Self::parse_encoding_decl(s)?;
        let standalone = Self::parse_standalone(s)?;

        s.skip_spaces();
        s.skip_string(b"?>")?;

        let span = s.slice_back(start);

        Ok(Token::Declaration { version, encoding, standalone, span })
    }

    // VersionInfo ::= S 'version' Eq ("'" VersionNum "'" | '"' VersionNum '"')
    // VersionNum  ::= '1.' [0-9]+
    fn parse_version_info(s: &mut Stream<'a>) -> StreamResult<StrSpan<'a>> {
        s.skip_spaces();
        s.skip_string(b"version")?;
        s.consume_eq()?;
        let quote = s.consume_quote()?;

        let start = s.pos();
        s.skip_string(b"1.")?;
        s.skip_bytes(|_, c| c.is_xml_digit());
        let ver = s.slice_back(start);

        s.consume_byte(quote)?;

        Ok(ver)
    }


    // EncodingDecl ::= S 'encoding' Eq ('"' EncName '"' | "'" EncName "'" )
    // EncName      ::= [A-Za-z] ([A-Za-z0-9._] | '-')*
    fn parse_encoding_decl(s: &mut Stream<'a>) -> StreamResult<Option<StrSpan<'a>>> {
        s.skip_spaces();

        if s.skip_string(b"encoding").is_err() {
            return Ok(None);
        }

        s.consume_eq()?;
        let quote = s.consume_quote()?;
        // [A-Za-z] ([A-Za-z0-9._] | '-')*
        // TODO: check that first byte is [A-Za-z]
        let name = s.consume_bytes(|_, c| {
               c.is_xml_letter()
            || c.is_xml_digit()
            || c == b'.'
            || c == b'-'
            || c == b'_'
        });
        s.consume_byte(quote)?;

        Ok(Some(name))
    }

    // SDDecl ::= S 'standalone' Eq (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no') '"'))
    fn parse_standalone(s: &mut Stream<'a>) -> StreamResult<Option<bool>> {
        s.skip_spaces();

        if s.skip_string(b"standalone").is_err() {
            return Ok(None);
        }

        s.consume_eq()?;
        let quote = s.consume_quote()?;

        let start = s.pos();
        let value = s.consume_name()?.as_str();

        let flag = match value {
            "yes" => true,
            "no" => false,
            _ => {
                let pos = s.gen_text_pos_from(start);

                return Err(StreamError::InvalidString("yes', 'no", pos));
            }
        };

        s.consume_byte(quote)?;

        Ok(Some(flag))
    }

    fn parse_comment(s: &mut Stream<'a>) -> Result<Token<'a>> {
        let start = s.pos() - 4;
        Self::parse_comment_impl(s)
            .map_err(|_| Error::InvalidToken(TokenType::Comment, s.gen_text_pos_from(start), None))
    }

    // '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
    fn parse_comment_impl(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        let start = s.pos() - 4;
        let text = s.consume_chars(|s, c| !(c == '-' && s.starts_with(b"-->")))?;
        s.skip_string(b"-->")?;

        if text.as_str().contains("--") || text.as_str().ends_with('-') {
            return Err(StreamError::UnexpectedEndOfStream); // Error type doesn't matter.
        }

        let span = s.slice_back(start);

        Ok(Token::Comment { text, span })
    }

    fn parse_pi(s: &mut Stream<'a>) -> Result<Token<'a>> {
        map_err_at!(Self::parse_pi_impl(s), TokenType::PI, s, -2)
    }

    // PI       ::= '<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>'
    // PITarget ::= Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))
    fn parse_pi_impl(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        let start = s.pos() - 2;
        let target = s.consume_name()?;
        s.skip_spaces();
        let content = s.consume_chars(|s, c| !(c == '?' && s.starts_with(b"?>")))?;
        let content = if !content.is_empty() {
            Some(content)
        } else {
            None
        };

        s.skip_string(b"?>")?;

        let span = s.slice_back(start);

        Ok(Token::ProcessingInstruction { target, content, span })
    }

    fn parse_doctype(s: &mut Stream<'a>) -> Result<Token<'a>> {
        map_err_at!(Self::parse_doctype_impl(s), TokenType::DoctypeDecl, s, -9)
    }

    // doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
    fn parse_doctype_impl(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        let start = s.pos() - 9;

        s.consume_spaces()?;
        let name = s.consume_name()?;
        s.skip_spaces();

        let external_id = Self::parse_external_id(s)?;
        s.skip_spaces();

        let c = s.curr_byte()?;
        if c != b'[' && c !=  b'>' {
            static EXPECTED: &[u8] = &[b'[', b'>'];
            return Err(StreamError::InvalidCharMultiple(c, EXPECTED, s.gen_text_pos()));
        }

        s.advance(1);

        let span = s.slice_back(start);
        if c == b'[' {
            Ok(Token::DtdStart { name, external_id, span })
        } else {
            Ok(Token::EmptyDtd { name, external_id, span })
        }
    }

    // ExternalID ::= 'SYSTEM' S SystemLiteral | 'PUBLIC' S PubidLiteral S SystemLiteral
    fn parse_external_id(s: &mut Stream<'a>) -> StreamResult<Option<ExternalId<'a>>> {
        let v = if s.starts_with(b"SYSTEM") || s.starts_with(b"PUBLIC") {
            let start = s.pos();
            s.advance(6);
            let id = s.slice_back(start);

            s.consume_spaces()?;
            let quote = s.consume_quote()?;
            let literal1 = s.consume_bytes(|_, c| c != quote);
            s.consume_byte(quote)?;

            let v = if id.as_str() == "SYSTEM" {
                ExternalId::System(literal1)
            } else {
                s.consume_spaces()?;
                let quote = s.consume_quote()?;
                let literal2 = s.consume_bytes(|_, c| c != quote);
                s.consume_byte(quote)?;

                ExternalId::Public(literal1, literal2)
            };

            Some(v)
        } else {
            None
        };

        Ok(v)
    }

    fn parse_entity_decl(s: &mut Stream<'a>) -> Result<Token<'a>> {
        map_err_at!(Self::parse_entity_decl_impl(s), TokenType::EntityDecl, s, -8)
    }

    // EntityDecl  ::= GEDecl | PEDecl
    // GEDecl      ::= '<!ENTITY' S Name S EntityDef S? '>'
    // PEDecl      ::= '<!ENTITY' S '%' S Name S PEDef S? '>'
    fn parse_entity_decl_impl(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        let start = s.pos() - 8;

        s.consume_spaces()?;

        let is_ge = if s.try_consume_byte(b'%') {
            s.consume_spaces()?;
            false
        } else {
            true
        };

        let name = s.consume_name()?;
        s.consume_spaces()?;
        let definition = Self::parse_entity_def(s, is_ge)?;
        s.skip_spaces();
        s.consume_byte(b'>')?;

        let span = s.slice_back(start);

        Ok(Token::EntityDeclaration { name, definition, span })
    }

    // EntityDef   ::= EntityValue | (ExternalID NDataDecl?)
    // PEDef       ::= EntityValue | ExternalID
    // EntityValue ::= '"' ([^%&"] | PEReference | Reference)* '"' |  "'" ([^%&']
    //                             | PEReference | Reference)* "'"
    // ExternalID  ::= 'SYSTEM' S SystemLiteral | 'PUBLIC' S PubidLiteral S SystemLiteral
    // NDataDecl   ::= S 'NDATA' S Name
    fn parse_entity_def(s: &mut Stream<'a>, is_ge: bool) -> StreamResult<EntityDefinition<'a>> {
        let c = s.curr_byte()?;
        match c {
            b'"' | b'\'' => {
                let quote = s.consume_quote()?;
                let value = s.consume_bytes(|_, c| c != quote);
                s.consume_byte(quote)?;

                Ok(EntityDefinition::EntityValue(value))
            }
            b'S' | b'P' => {
                if let Some(id) = Self::parse_external_id(s)? {
                    if is_ge {
                        s.skip_spaces();
                        if s.starts_with(b"NDATA") {
                            s.advance(5);
                            s.consume_spaces()?;
                            s.skip_name()?;
                            // TODO: NDataDecl is not supported
                        }
                    }

                    Ok(EntityDefinition::ExternalId(id))
                } else {
                    Err(StreamError::InvalidExternalID)
                }
            }
            _ => {
                static EXPECTED: &[u8] = &[b'"', b'\'', b'S', b'P'];
                let pos = s.gen_text_pos();
                Err(StreamError::InvalidCharMultiple(c, EXPECTED, pos))
            }
        }
    }

    fn consume_decl(s: &mut Stream) -> StreamResult<()> {
        s.consume_spaces()?;
        s.skip_bytes(|_, c| c != b'>');
        s.consume_byte(b'>')?;
        Ok(())
    }

    fn parse_cdata(s: &mut Stream<'a>) -> Result<Token<'a>> {
        map_err_at!(Self::parse_cdata_impl(s), TokenType::CDSect, s, -9)
    }

    // CDSect  ::= CDStart CData CDEnd
    // CDStart ::= '<![CDATA['
    // CData   ::= (Char* - (Char* ']]>' Char*))
    // CDEnd   ::= ']]>'
    fn parse_cdata_impl(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        let start = s.pos() - 9;
        let text = s.consume_chars(|s, c| !(c == ']' && s.starts_with(b"]]>")))?;
        s.skip_string(b"]]>")?;
        let span = s.slice_back(start);
        Ok(Token::Cdata { text, span })
    }

    fn parse_element_start(s: &mut Stream<'a>) -> Result<Token<'a>> {
        map_err_at!(Self::parse_element_start_impl(s), TokenType::ElementStart, s, -1)
    }

    // '<' Name (S Attribute)* S? '>'
    fn parse_element_start_impl(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        let start = s.pos() - 1;
        let (prefix, local) = s.consume_qname()?;
        let span = s.slice_back(start);

        Ok(Token::ElementStart { prefix, local, span })
    }

    fn parse_close_element(s: &mut Stream<'a>) -> Result<Token<'a>> {
        map_err_at!(Self::parse_close_element_impl(s), TokenType::ElementClose, s, -2)
    }

    // '</' Name S? '>'
    fn parse_close_element_impl(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        let start = s.pos() - 2;

        let (prefix, tag_name) = s.consume_qname()?;
        s.skip_spaces();
        s.consume_byte(b'>')?;

        let span = s.slice_back(start);

        Ok(Token::ElementEnd { end: ElementEnd::Close(prefix, tag_name), span })
    }

    // Name Eq AttValue
    fn parse_attribute(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        s.skip_spaces();

        if let Ok(c) = s.curr_byte() {
            let start = s.pos();

            match c {
                b'/' => {
                    s.advance(1);
                    s.consume_byte(b'>')?;
                    let span = s.slice_back(start);
                    return Ok(Token::ElementEnd { end: ElementEnd::Empty, span });
                }
                b'>' => {
                    s.advance(1);
                    let span = s.slice_back(start);
                    return Ok(Token::ElementEnd { end: ElementEnd::Open, span });
                }
                _ => {}
            }
        }

        let start = s.pos();

        let (prefix, local) = s.consume_qname()?;
        s.consume_eq()?;
        let quote = s.consume_quote()?;
        let quote_c = quote as char;
        // The attribute value must not contain the < character.
        let value = s.consume_chars(|_, c| c != quote_c && c != '<')?;
        s.consume_byte(quote)?;
        let span = s.slice_back(start);

        s.skip_spaces();

        Ok(Token::Attribute { prefix, local, value, span })
    }

    fn parse_text(s: &mut Stream<'a>) -> Result<Token<'a>> {
        map_err_at!(Self::parse_text_impl(s), TokenType::CharData, s, 0)
    }

    fn parse_text_impl(s: &mut Stream<'a>) -> StreamResult<Token<'a>> {
        let text = s.consume_chars(|_, c| c != '<')?;

        // According to the spec, `]]>` must not appear inside a Text node.
        // https://www.w3.org/TR/xml/#syntax
        //
        // Search for `>` first, since it's a bit faster than looking for `]]>`.
        if text.as_str().contains('>') {
            if text.as_str().contains("]]>") {
                return Err(StreamError::InvalidCharacterData);
            }
        }

        Ok(Token::Text { text })
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.stream.at_end() || self.state == State::End {
            return None;
        }

        let t = Self::parse_next_impl(&mut self.stream, self.state);

        if let Some(ref t) = t {
            match *t {
                Ok(t) => match t {
                    Token::ElementStart { .. } => {
                        self.state = State::Attributes;
                    }
                    Token::ElementEnd { ref end, .. } => {
                        match *end {
                            ElementEnd::Open => self.depth += 1,
                            ElementEnd::Close(..) if self.depth > 0 => self.depth -= 1,
                            _ => {}
                        }

                        if self.depth == 0 && !self.fragment_parsing {
                            self.state = State::AfterElements;
                        } else {
                            self.state = State::Elements;
                        }
                    }
                    Token::DtdStart { .. } => {
                        self.state = State::Dtd;
                    }
                    Token::EmptyDtd { .. } | Token::DtdEnd { .. } => {
                        self.state = State::AfterDtd;
                    }
                    _ => {}
                },
                Err(_) => {
                    self.stream.jump_to_end();
                    self.state = State::End;
                }
            }
        }

        t
    }
}
