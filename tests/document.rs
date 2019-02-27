extern crate xmlparser as xml;

use std::str;

#[macro_use] mod token;
use token::*;

test!(document_01, "", );

test!(document_02, "    ", );

test!(document_03, " \n\t\r ", );

// BOM
test!(document_05, str::from_utf8(b"\xEF\xBB\xBF<a/>").unwrap(),
    Token::ElementStart("", "a", 3..5),
    Token::ElementEnd(ElementEnd::Empty, 5..7)
);

test!(document_06, str::from_utf8(b"\xEF\xBB\xBF<?xml version='1.0'?>").unwrap(),
    Token::Declaration("1.0", None, None, 3..24)
);

test!(document_err_01, "<![CDATA[text]]>",
    Token::Error("unexpected token 'CDATA' at 1:1".to_string())
);

test!(document_err_02, " &www---------Ӥ+----------w-----www_",
    Token::Error("unknown token at 1:2".to_string())
);

test!(document_err_03, "q",
    Token::Error("unknown token at 1:1".to_string())
);

test!(document_err_04, "<!>",
    Token::Error("unknown token at 1:1".to_string())
);

test!(document_err_05, "<!DOCTYPE greeting1><!DOCTYPE greeting2>",
    Token::EmptyDtd("greeting1", None, 0..20),
    Token::Error("unexpected token 'Doctype Declaration' at 1:21".to_string())
);

test!(document_err_06, "&#x20;",
    Token::Error("unknown token at 1:1".to_string())
);

#[test]
fn parse_fragment_1() {
    let s = "<p/><p/>";
    let mut p = xml::Tokenizer::from(s);
    p.enable_fragment_mode();

    match p.next().unwrap().unwrap() {
        xml::Token::ElementStart { local, .. } => assert_eq!(local.as_str(), "p"),
        _ => panic!(),
    }

    match p.next().unwrap().unwrap() {
        xml::Token::ElementEnd { .. } => {}
        _ => panic!(),
    }

    match p.next().unwrap().unwrap() {
        xml::Token::ElementStart { local, .. } => assert_eq!(local.as_str(), "p"),
        _ => panic!(),
    }
}
