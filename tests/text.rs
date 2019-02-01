extern crate xmlparser as xml;

#[macro_use] mod token;
use token::*;

test!(text_01, "<p>text</p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("text", 3..7),
    Token::ElementEnd(ElementEnd::Close("", "p"), 7..11)
);

test!(text_02, "<p> text </p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text(" text ", 3..9),
    Token::ElementEnd(ElementEnd::Close("", "p"), 9..13)
);

// 欄 is EF A4 9D. And EF can be mistreated for UTF-8 BOM.
test!(text_03, "<p>欄</p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("欄", 3..6),
    Token::ElementEnd(ElementEnd::Close("", "p"), 6..10)
);

test!(whitespaces_01, "<p> </p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Whitespaces(" ", 3..4),
    Token::ElementEnd(ElementEnd::Close("", "p"), 4..8)
);

test!(whitespaces_02, "<p> \r\n\t </p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Whitespaces(" \r\n\t ", 3..8),
    Token::ElementEnd(ElementEnd::Close("", "p"), 8..12)
);

test!(whitespaces_03, "<p>&#x20;</p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Whitespaces("&#x20;", 3..9),
    Token::ElementEnd(ElementEnd::Close("", "p"), 9..13)
);

test!(whitespaces_04, "<p>&#x9;&#xA;&#xD;&#x20;</p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Whitespaces("&#x9;&#xA;&#xD;&#x20;", 3..24),
    Token::ElementEnd(ElementEnd::Close("", "p"), 24..28)
);
