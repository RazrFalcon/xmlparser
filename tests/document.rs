extern crate xmlparser as xml;

use std::str;

#[macro_use] mod token;
use token::*;

test!(document_01, "", );

test!(document_02, "    ", );

test!(document_03, " \n\t\r ", );

test!(document_04, "&#x20;", );

// BOM
test!(document_05, str::from_utf8(b"\xEF\xBB\xBF<a/>").unwrap(),
    Token::ElementStart("", "a"),
    Token::ElementEnd(ElementEnd::Empty)
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

#[test]
fn parse_fragment_1() {
    let s = "<p/><p/>";
    let mut p = xml::Tokenizer::from(s);
    p.set_fragment_mode();

    match p.next().unwrap().unwrap() {
        xml::Token::ElementStart(_, local) => assert_eq!(local.to_str(), "p"),
        _ => panic!(),
    }

    match p.next().unwrap().unwrap() {
        xml::Token::ElementEnd(_) => {}
        _ => panic!(),
    }

    match p.next().unwrap().unwrap() {
        xml::Token::ElementStart(_, local) => assert_eq!(local.to_str(), "p"),
        _ => panic!(),
    }
}
