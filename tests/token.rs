extern crate xmlparser as xml;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Declaration(&'a str, Option<&'a str>, Option<&'a str>),
    PI(&'a str, Option<&'a str>),
    Comment(&'a str),
    DtdStart(&'a str, Option<ExternalId<'a>>),
    EmptyDtd(&'a str, Option<ExternalId<'a>>),
    EntityDecl(&'a str, EntityDefinition<'a>),
    DtdEnd,
    ElementStart(&'a str, &'a str),
    Attribute(&'a str, &'a str, &'a str),
    ElementEnd(ElementEnd<'a>),
    Text(&'a str),
    Whitespaces(&'a str),
    Cdata(&'a str),
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
            let mut p = xml::Tokenizer::from($text);
            $(
                assert_eq!(to_test_token(p.next().unwrap()), $token);
            )*
            assert!(p.next().is_none());
        }
    )
}

#[inline(never)]
pub fn to_test_token(token: Result<xml::Token, xml::Error>) -> Token {
    match token {
        Ok(xml::Token::Declaration(version, encoding, standalone)) => {
            Token::Declaration(
                version.to_str(),
                encoding.map(|v| v.to_str()),
                standalone.map(|v| v.to_str()),
            )
        }
        Ok(xml::Token::ProcessingInstruction(target, content)) => {
            Token::PI(
                target.to_str(),
                content.map(|v| v.to_str()),
            )
        }
        Ok(xml::Token::Comment(text)) => Token::Comment(text.to_str()),
        Ok(xml::Token::DtdStart(name, external_id)) => {
            Token::DtdStart(
                name.to_str(),
                external_id.map(|v| to_test_external_id(v)),
            )
        }
        Ok(xml::Token::EmptyDtd(name, external_id)) => {
            Token::EmptyDtd(
                name.to_str(),
                external_id.map(|v| to_test_external_id(v)),
            )
        }
        Ok(xml::Token::EntityDeclaration(name, def)) => {
            Token::EntityDecl(
                name.to_str(),
                match def {
                    xml::EntityDefinition::EntityValue(name) => {
                        EntityDefinition::EntityValue(name.to_str())
                    }
                    xml::EntityDefinition::ExternalId(id) => {
                        EntityDefinition::ExternalId(to_test_external_id(id))
                    }
                }
            )
        }
        Ok(xml::Token::DtdEnd) => Token::DtdEnd,
        Ok(xml::Token::ElementStart(prefix, local)) => {
            Token::ElementStart(
                prefix.to_str(),
                local.to_str(),
            )
        }
        Ok(xml::Token::Attribute((prefix, local), value)) => {
            Token::Attribute(
                prefix.to_str(), local.to_str(), value.to_str()
            )
        }
        Ok(xml::Token::ElementEnd(end)) => {
            Token::ElementEnd(
                match end {
                    xml::ElementEnd::Open => ElementEnd::Open,
                    xml::ElementEnd::Close(prefix, local) => {
                        ElementEnd::Close(prefix.to_str(), local.to_str())
                    }
                    xml::ElementEnd::Empty => ElementEnd::Empty,
                }
            )
        }
        Ok(xml::Token::Text(text)) => Token::Text(text.to_str()),
        Ok(xml::Token::Whitespaces(text)) => Token::Whitespaces(text.to_str()),
        Ok(xml::Token::Cdata(text)) => Token::Cdata(text.to_str()),
        Err(ref e) => Token::Error(e.to_string()),
    }
}

fn to_test_external_id(id: xml::ExternalId) -> ExternalId {
    match id {
        xml::ExternalId::System(name) => {
            ExternalId::System(name.to_str())
        }
        xml::ExternalId::Public(name, value) => {
            ExternalId::Public(name.to_str(), value.to_str())
        }
    }
}
