extern crate xmlparser;

use xmlparser::*;

#[test]
fn trim_1() {
    assert_eq!(StrSpan::from("  text  ").trim().to_str(), "text");
}

#[test]
fn trim_2() {
    assert_eq!(StrSpan::from("  text  text  ").trim().to_str(), "text  text");
}

#[test]
fn trim_3() {
    assert_eq!(StrSpan::from("&#x20;text&#x20;").trim().to_str(), "text");
}

#[test]
fn trim_4() {
    assert_eq!(StrSpan::from("&#x20;text&#x20;text&#x20;").trim().to_str(), "text&#x20;text");
}

#[test]
fn do_not_trim_1() {
    assert_eq!(StrSpan::from("&#x40;text&#x50;").trim().to_str(), "&#x40;text&#x50;");
}

#[test]
fn do_not_trim_2() {
    assert_eq!(StrSpan::from("&ref;text&apos;").trim().to_str(), "&ref;text&apos;");
}


#[test]
fn text_pos_1() {
    let mut s = Stream::from("text");
    s.advance(2);
    assert_eq!(s.gen_text_pos(), TextPos::new(1, 3));
}

#[test]
fn text_pos_2() {
    let mut s = Stream::from("text\ntext");
    s.advance(6);
    assert_eq!(s.gen_text_pos(), TextPos::new(2, 2));
}

#[test]
fn text_pos_3() {
    let mut s = Stream::from("текст\nтекст");
    s.advance(15);
    assert_eq!(s.gen_text_pos(), TextPos::new(2, 3));
}


#[test]
fn token_size() {
    assert!(::std::mem::size_of::<Token>() <= 196);
}

#[test]
fn err_size_1() {
    assert!(::std::mem::size_of::<Error>() <= 64);
}

#[test]
fn err_size_2() {
    assert!(::std::mem::size_of::<StreamError>() <= 64);
}
