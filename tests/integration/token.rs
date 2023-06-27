type Range = ::std::ops::Range<usize>;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Declaration(&'a str, Option<&'a str>, Option<bool>, Range),
    PI(&'a str, Option<&'a str>, Range),
    ConditionalCommentStart(&'a str, Range),
    ConditionalCommentEnd(Range),
    Comment(&'a str, Range),
    DtdStart(&'a str, Option<ExternalId<'a>>, Range),
    EmptyDtd(&'a str, Option<ExternalId<'a>>, Range),
    EntityDecl(&'a str, EntityDefinition<'a>, Range),
    DtdEnd(Range),
    ElementStart(&'a str, &'a str, Range),
    Attribute(&'a str, &'a str, &'a str, Range),
    ElementEnd(ElementEnd<'a>, Range),
    Text(&'a str, Range),
    Cdata(&'a str, Range),
    Error(String),
}

#[derive(PartialEq, Debug)]
pub enum ElementEnd<'a> {
    Open,
    Close(&'a str, &'a str),
    Empty,
}

#[derive(PartialEq, Debug)]
pub enum ExternalId<'a> {
    System(&'a str),
    Public(&'a str, &'a str),
}

#[derive(PartialEq, Debug)]
pub enum EntityDefinition<'a> {
    EntityValue(&'a str),
    ExternalId(ExternalId<'a>),
}

#[macro_export]
macro_rules! test {
    ($name:ident, $text:expr, $($token:expr),*) => (
        #[test]
        fn $name() {
            let mut p = html::Tokenizer::from($text);
            $(
                let t = p.next().unwrap();
                assert_eq!(to_test_token(t), $token);
            )*
            assert!(p.next().is_none());
        }
    )
}

#[inline(never)]
pub fn to_test_token(token: Result<html::Token, html::Error>) -> Token {
    match token {
        Ok(html::Token::Declaration {
            version,
            encoding,
            standalone,
            span,
        }) => Token::Declaration(
            version.as_str(),
            encoding.map(|v| v.as_str()),
            standalone,
            span.range(),
        ),
        Ok(html::Token::ProcessingInstruction {
            target,
            content,
            span,
        }) => Token::PI(target.as_str(), content.map(|v| v.as_str()), span.range()),
        Ok(html::Token::ConditionalCommentStart { condition, span }) => {
            Token::ConditionalCommentStart(condition.as_str(), span.range())
        }
        Ok(html::Token::ConditionalCommentEnd { span }) => {
            Token::ConditionalCommentEnd(span.range())
        }
        Ok(html::Token::Comment { text, span }) => Token::Comment(text.as_str(), span.range()),
        Ok(html::Token::DtdStart {
            name,
            external_id,
            span,
        }) => Token::DtdStart(
            name.as_str(),
            external_id.map(to_test_external_id),
            span.range(),
        ),
        Ok(html::Token::EmptyDtd {
            name,
            external_id,
            span,
        }) => Token::EmptyDtd(
            name.as_str(),
            external_id.map(to_test_external_id),
            span.range(),
        ),
        Ok(html::Token::EntityDeclaration {
            name,
            definition,
            span,
        }) => Token::EntityDecl(
            name.as_str(),
            match definition {
                html::EntityDefinition::EntityValue(name) => {
                    EntityDefinition::EntityValue(name.as_str())
                }
                html::EntityDefinition::ExternalId(id) => {
                    EntityDefinition::ExternalId(to_test_external_id(id))
                }
            },
            span.range(),
        ),
        Ok(html::Token::DtdEnd { span }) => Token::DtdEnd(span.range()),
        Ok(html::Token::ElementStart {
            prefix,
            local,
            span,
        }) => Token::ElementStart(prefix.as_str(), local.as_str(), span.range()),
        Ok(html::Token::Attribute {
            prefix,
            local,
            value,
            span,
        }) => Token::Attribute(
            prefix.as_str(),
            local.as_str(),
            value.as_str(),
            span.range(),
        ),
        Ok(html::Token::ElementEnd { end, span }) => Token::ElementEnd(
            match end {
                html::ElementEnd::Open => ElementEnd::Open,
                html::ElementEnd::Close(prefix, local) => {
                    ElementEnd::Close(prefix.as_str(), local.as_str())
                }
                html::ElementEnd::Empty => ElementEnd::Empty,
            },
            span.range(),
        ),
        Ok(html::Token::Text { text }) => Token::Text(text.as_str(), text.range()),
        Ok(html::Token::Cdata { text, span }) => Token::Cdata(text.as_str(), span.range()),
        Err(ref e) => Token::Error(e.to_string()),
    }
}

fn to_test_external_id(id: html::ExternalId) -> ExternalId {
    match id {
        html::ExternalId::System(name) => ExternalId::System(name.as_str()),
        html::ExternalId::Public(name, value) => ExternalId::Public(name.as_str(), value.as_str()),
    }
}
