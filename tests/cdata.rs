extern crate xmlparser as xml;

#[macro_use] mod token;
use token::*;

test!(cdata_01, "<p><![CDATA[content]]></p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Cdata("content"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(cdata_02, "<p><![CDATA[&amping]]></p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Cdata("&amping"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(cdata_03, "<p><![CDATA[&amping ]]]></p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Cdata("&amping ]"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(cdata_04, "<p><![CDATA[&amping]] ]]></p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Cdata("&amping]] "),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(cdata_05, "<p><![CDATA[<message>text</message>]]></p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Cdata("<message>text</message>"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(cdata_06, "<p><![CDATA[</this is malformed!</malformed</malformed & worse>]]></p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Cdata("</this is malformed!</malformed</malformed & worse>"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(cdata_07, "<p><![CDATA[1]]><![CDATA[2]]></p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Cdata("1"),
    Token::Cdata("2"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(cdata_08, "<p> \n <![CDATA[data]]> \t </p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Whitespaces(" \n "),
    Token::Cdata("data"),
    Token::Whitespaces(" \t "),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);

test!(cdata_09, "<p><![CDATA[bracket ]after]]></p>",
    Token::ElementStart("", "p"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Cdata("bracket ]after"),
    Token::ElementEnd(ElementEnd::Close("", "p"))
);
