extern crate xmlparser as xml;

#[macro_use] mod token;
use token::*;

test!(element_01, "<a/>",
    Token::ElementStart("", "a"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(element_02, "<a></a>",
    Token::ElementStart("", "a"),
    Token::ElementEnd(ElementEnd::Open),
    Token::ElementEnd(ElementEnd::Close("", "a"))
);

test!(element_03, "  \t  <a/>   \n ",
    Token::ElementStart("", "a"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(element_04, "  \t  <b><a/></b>   \n ",
    Token::ElementStart("", "b"),
    Token::ElementEnd(ElementEnd::Open),
    Token::ElementStart("", "a"),
    Token::ElementEnd(ElementEnd::Empty),
    Token::ElementEnd(ElementEnd::Close("", "b"))
);

test!(element_05, "&#x9;&#xA;&#xD;&#x20;<b/>&#x9;&#xA;&#xD;&#x20;",
    Token::ElementStart("", "b"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(element_06, "<俄语 լեզու=\"ռուսերեն\">данные</俄语>",
    Token::ElementStart("", "俄语"),
    Token::Attribute("", "լեզու", "ռուսերեն"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Text("данные"),
    Token::ElementEnd(ElementEnd::Close("", "俄语"))
);

test!(element_07, "<svg:circle></svg:circle>",
    Token::ElementStart("svg", "circle"),
    Token::ElementEnd(ElementEnd::Open),
    Token::ElementEnd(ElementEnd::Close("svg", "circle"))
);

test!(element_08, "<:circle/>",
    Token::ElementStart("", "circle"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(element_err_01, "<>",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_02, "</",
    Token::Error("unexpected token 'Element Close' at 1:1".to_string())
);

test!(element_err_03, "</a",
    Token::Error("unexpected token 'Element Close' at 1:1".to_string())
);

test!(element_err_04, "<a x='test' /",
    Token::ElementStart("", "a"),
    Token::Attribute("", "x", "test"),
    Token::Error("invalid token 'Attribute' at 1:13 cause unexpected end of stream".to_string())
);

test!(element_err_05, "<<",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_06, "< a",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_07, "< ",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_08, "<&#x9;",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_09, "<a></a></a>",
    Token::ElementStart("", "a"),
    Token::ElementEnd(ElementEnd::Open),
    Token::ElementEnd(ElementEnd::Close("", "a")),
    Token::Error("unexpected token 'Element Close' at 1:8".to_string())
);

test!(element_err_10, "<a/><a/>",
    Token::ElementStart("", "a"),
    Token::ElementEnd(ElementEnd::Empty),
    Token::Error("unexpected token 'Element Start' at 1:5".to_string())
);

test!(element_err_11, "<a></br/></a>",
    Token::ElementStart("", "a"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Error("invalid token 'Element Close' at 1:4 cause expected '>' not '/' at 1:8".to_string())
);

test!(element_err_12, "<svg:/>",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_13, "\
<root>
</root>
</root>",
    Token::ElementStart("", "root"),
    Token::ElementEnd(ElementEnd::Open),
    Token::Whitespaces("\n"),
    Token::ElementEnd(ElementEnd::Close("", "root")),
    Token::Error("unexpected token 'Element Close' at 3:1".to_string())
);


test!(attribute_01, "<a ax=\"test\"/>",
    Token::ElementStart("", "a"),
    Token::Attribute("", "ax", "test"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(attribute_02, "<a ax='test'/>",
    Token::ElementStart("", "a"),
    Token::Attribute("", "ax", "test"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(attribute_03, "<a b='test1' c=\"test2\"/>",
    Token::ElementStart("", "a"),
    Token::Attribute("", "b", "test1"),
    Token::Attribute("", "c", "test2"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(attribute_04, "<a b='\"test1\"' c=\"'test2'\"/>",
    Token::ElementStart("", "a"),
    Token::Attribute("", "b", "\"test1\""),
    Token::Attribute("", "c", "'test2'"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(attribute_05, "<c a=\"test1' c='test2\" b='test1\" c=\"test2'/>",
    Token::ElementStart("", "c"),
    Token::Attribute("", "a", "test1' c='test2"),
    Token::Attribute("", "b", "test1\" c=\"test2"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(attribute_06, "<c   a   =    'test1'     />",
    Token::ElementStart("", "c"),
    Token::Attribute("", "a", "test1"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(attribute_07, "<c q:a='b'/>",
    Token::ElementStart("", "c"),
    Token::Attribute("q", "a", "b"),
    Token::ElementEnd(ElementEnd::Empty)
);

test!(attribute_err_01, "<c az=test>",
    Token::ElementStart("", "c"),
    Token::Error("invalid token 'Attribute' at 1:3 cause expected quote mark not 't' at 1:7".to_string())
);

test!(attribute_err_02, "<c a>",
    Token::ElementStart("", "c"),
    Token::Error("invalid token 'Attribute' at 1:3 cause expected \'=\' not \'>\' at 1:5".to_string())
);

test!(attribute_err_03, "<c a/>",
    Token::ElementStart("", "c"),
    Token::Error("invalid token 'Attribute' at 1:3 cause expected '=' not '/' at 1:5".to_string())
);

test!(attribute_err_04, "<c a='b' q/>",
    Token::ElementStart("", "c"),
    Token::Attribute("", "a", "b"),
    Token::Error("invalid token 'Attribute' at 1:10 cause expected '=' not '/' at 1:11".to_string())
);

test!(attribute_err_05, "<c a='<'/>",
    Token::ElementStart("", "c"),
    Token::Error("invalid token 'Attribute' at 1:3 cause attribute value with '<' character is not allowed".to_string())
);
