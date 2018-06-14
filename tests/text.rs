extern crate xmlparser as xml;

#[macro_use] mod token;
use token::*;

test!(text_01, "<p>text</p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Text("text"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(text_02, "<p> text </p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Text(" text "),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(whitespaces_01, "<p> </p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Whitespaces(" "),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(whitespaces_02, "<p> \r\n\t </p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Whitespaces(" \r\n\t "),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(whitespaces_03, "<p>&#x20;</p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Whitespaces("&#x20;"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(whitespaces_04, "<p>&#x9;&#xA;&#xD;&#x20;</p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Whitespaces("&#x9;&#xA;&#xD;&#x20;"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);
