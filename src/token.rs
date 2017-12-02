use {
    StrSpan,
};


/// An XML token.
#[derive(Debug)]
pub enum Token<'a> {
    /// Declaration token.
    ///
    /// Example: `<?xml version="1.0"?>`
    Declaration(StrSpan<'a>, Option<StrSpan<'a>>, Option<StrSpan<'a>>),
    /// Processing instruction token.
    ///
    /// Example: `<?target content?>`
    ProcessingInstruction(StrSpan<'a>, Option<StrSpan<'a>>),
    /// The comment token.
    ///
    /// Example: `<!-- text -->`
    Comment(StrSpan<'a>),
    /// DOCTYPE start token.
    ///
    /// Example: `<!DOCTYPE note [`
    DtdStart(StrSpan<'a>, Option<ExternalId<'a>>),
    /// Empty DOCTYPE token.
    ///
    /// Example: `<!DOCTYPE note>`
    EmptyDtd(StrSpan<'a>, Option<ExternalId<'a>>),
    /// ENTITY token.
    ///
    /// Can appear only inside the DTD.
    ///
    /// Example: `<!ENTITY ns_extend "http://test.com">`
    EntityDecl(StrSpan<'a>, EntityDefinition<'a>),
    /// DOCTYPE end token.
    ///
    /// Example: `]>`
    DtdEnd,
    /// Element start token.
    ///
    /// Example: `<elem`
    ElementStart(StrSpan<'a>),
    /// Attribute.
    ///
    /// Example: `name="value"`
    Attribute(StrSpan<'a>, StrSpan<'a>),
    /// Element end token.
    ElementEnd(ElementEnd<'a>),
    /// Text token.
    ///
    /// Contains text between elements including whitespaces.
    /// Basically everything between `>` and `<`.
    ///
    /// Contains text as is. Use [`TextUnescape`] to unescape it.
    ///
    /// Example: `<text>text</text>`
    ///
    /// [`TextUnescape`]: struct.TextUnescape.html
    Text(StrSpan<'a>),
    /// Whitespaces token.
    ///
    /// The same as `Text` token, but contains only spaces.
    ///
    /// Spaces can be encoded like `&#x20`.
    Whitespaces(StrSpan<'a>),
    /// CDATA token.
    ///
    /// Example: `<![CDATA[text]]>`
    Cdata(StrSpan<'a>),
}


/// `ElementEnd` token.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ElementEnd<'a> {
    /// Indicates `>`
    Open,
    /// Indicates `</name>`
    Close(StrSpan<'a>),
    /// Indicates `/>`
    Empty,
}


/// Representation of the [ExternalID](https://www.w3.org/TR/xml/#NT-ExternalID) value.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum ExternalId<'a> {
    System(StrSpan<'a>),
    Public(StrSpan<'a>, StrSpan<'a>),
}


/// Representation of the [EntityDef](https://www.w3.org/TR/xml/#NT-EntityDef) value.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum EntityDefinition<'a> {
    EntityValue(StrSpan<'a>),
    ExternalId(ExternalId<'a>),
}
