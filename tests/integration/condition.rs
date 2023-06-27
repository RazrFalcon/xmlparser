use crate::token::*;

#[test]
fn conditional_comment_in_html() {
    let tokenizer = html::Tokenizer::from(
        "<!DOCTYPE html><html><head><!--[if IE 8]><style></style><![endif]--></head></html>",
    );
    let result: Vec<Token> = tokenizer.map(to_test_token).collect();
    assert_eq!(
        result,
        vec![
            Token::EmptyDtd("html", None, 0..15),
            Token::ElementStart("", "html", 15..20),
            Token::ElementEnd(ElementEnd::Open, 20..21),
            Token::ElementStart("", "head", 21..26),
            Token::ElementEnd(ElementEnd::Open, 26..27),
            Token::ConditionalCommentStart("if IE 8", 27..41),
            Token::ElementStart("", "style", 41..47),
            Token::ElementEnd(ElementEnd::Open, 47..48),
            Token::ElementEnd(ElementEnd::Close("", "style"), 48..56),
            Token::ConditionalCommentEnd(56..68),
            Token::ElementEnd(ElementEnd::Close("", "head"), 68..75),
            Token::ElementEnd(ElementEnd::Close("", "html"), 75..82)
        ]
    );
}

test!(
    conditional_comment_start_01,
    "<!--[if IE 8]>",
    Token::ConditionalCommentStart("if IE 8", 0..14)
);
test!(
    conditional_comment_start_02,
    "<!--[if lte IE 7]>",
    Token::ConditionalCommentStart("if lte IE 7", 0..18)
);
test!(
    conditional_comment_start_03,
    "<![if !IE]>",
    Token::ConditionalCommentStart("if !IE", 0..11)
);
test!(
    conditional_comment_start_04,
    "<!--[if !IE]>-->",
    Token::ConditionalCommentStart("if !IE", 0..16)
);
test!(
    conditional_comment_start_05,
    "<!--[if gt IE 6]><!-->",
    Token::ConditionalCommentStart("if gt IE 6", 0..22)
);
test!(
    conditional_comment_end_01,
    "<![endif]-->",
    Token::ConditionalCommentEnd(0..12)
);
test!(
    conditional_comment_end_02,
    "<![endif]>",
    Token::ConditionalCommentEnd(0..10)
);
test!(
    conditional_comment_end_03,
    "<!--<![endif]-->",
    Token::ConditionalCommentEnd(0..16)
);
