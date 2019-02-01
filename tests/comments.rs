extern crate xmlparser as xml;

#[macro_use] mod token;
use token::*;

test!(comment_01, "<!--comment-->",     Token::Comment("comment", 0..14));
test!(comment_02, "<!--<head>-->",      Token::Comment("<head>", 0..13));
test!(comment_03, "<!----->",           Token::Comment("-", 0..8));
test!(comment_04, "<!--<!-x-->",        Token::Comment("<!-x", 0..11));
test!(comment_05, "<!--<!x-->",         Token::Comment("<!x", 0..10));
test!(comment_06, "<!--<<!x-->",        Token::Comment("<<!x", 0..11));
test!(comment_07, "<!--<<!-x-->",       Token::Comment("<<!-x", 0..12));
test!(comment_08, "<!--<x-->",          Token::Comment("<x", 0..9));
test!(comment_09, "<!--<>-->",          Token::Comment("<>", 0..9));
test!(comment_10, "<!--<-->",           Token::Comment("<", 0..8));
test!(comment_11, "<!--<--->",          Token::Comment("<-", 0..9));
test!(comment_12, "<!--<!-->",          Token::Comment("<!", 0..9));
test!(comment_13, "<!---->",            Token::Comment("", 0..7));

macro_rules! test_err {
    ($name:ident, $text:expr) => (
        #[test]
        fn $name() {
            let mut p = xml::Tokenizer::from($text);
            assert_eq!(p.next().unwrap().unwrap_err().to_string(),
                       "invalid token 'Comment' at 1:1");
        }
    )
}

test_err!(comment_err_01, "<!----!>");
test_err!(comment_err_02, "<!----!");
test_err!(comment_err_03, "<!----");
test_err!(comment_err_04, "<!--->");
test_err!(comment_err_05, "<!-----");
test_err!(comment_err_06, "<!-->");
test_err!(comment_err_07, "<!--");
test_err!(comment_err_08, "<!--x");
test_err!(comment_err_09, "<!--<");
test_err!(comment_err_10, "<!--<!");
test_err!(comment_err_11, "<!--<!-");
test_err!(comment_err_12, "<!--<!--");
test_err!(comment_err_13, "<!--<!--!");
test_err!(comment_err_14, "<!--<!--!>");
test_err!(comment_err_15, "<!--<!---");
test_err!(comment_err_16, "<!--<!--x");
test_err!(comment_err_17, "<!--<!--x-");
test_err!(comment_err_18, "<!--<!--x--");
test_err!(comment_err_19, "<!--<!--x-->");
test_err!(comment_err_20, "<!--<!-x");
test_err!(comment_err_21, "<!--<!-x-");
test_err!(comment_err_22, "<!--<!-x--");
test_err!(comment_err_23, "<!--<!x");
test_err!(comment_err_24, "<!--<!x-");
test_err!(comment_err_25, "<!--<!x--");
test_err!(comment_err_26, "<!--<<!--x-->");
test_err!(comment_err_27, "<!--<!<!--x-->");
test_err!(comment_err_28, "<!--<!-<!--x-->");
test_err!(comment_err_29, "<!----!->");
test_err!(comment_err_30, "<!----!x>");
test_err!(comment_err_31, "<!-----x>");
