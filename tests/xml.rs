extern crate rustc_test;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate xmlparser;

use std::env;
use std::fs;
use std::str;
use std::path::Path;

use xmlparser as xml;
use xmlparser::{FromSpan, ChainedError};

use rustc_test::{TestDesc, TestDescAndFn, DynTestName, DynTestFn};

macro_rules! assert_eq_text {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `(left == right)` \
                           \nleft:  `{}`\nright: `{}`",
                           left_val, right_val)
                }
            }
        }
    })
}

mod tst {

#[derive(Deserialize, Clone, Debug)]
pub enum ElementEnd {
    Open,
    Close(String),
    Empty,
}

#[derive(Deserialize, Clone, Debug)]
pub enum Token {
    ElementStart(String),
    Attribute(String, String),
    ElementEnd(ElementEnd),
    PI(String, Option<String>),
    Declaration(String, Option<String>, Option<String>),
    Text(String),
    Whitespaces(String),
    Comment(String),
    CDATA(String),
    EmptyDTD(String, String, String, String),
    DTDStart(String, String, String, String),
    EntityDecl(String, String, String, String),
    DTDEnd,
    Error(String),
}

#[derive(Deserialize, Clone)]
pub struct TestData {
    pub description: String,
    pub input: String,
    pub output: Vec<Token>,
}

#[derive(Deserialize)]
pub struct Tests {
    pub tests: Vec<TestData>,
}

}

trait HasExtension {
    fn has_extension(&self, ext: &str) -> bool;
}

impl HasExtension for Path {
    fn has_extension(&self, ext: &str) -> bool {
        if let Some(e) = self.extension() {
            e == ext
        } else {
            false
        }
    }
}

#[test]
fn run() {
    let mut tests = vec![];

    for entry in fs::read_dir("tests").unwrap() {
        let entry = entry.unwrap();

        if !entry.path().has_extension("json") {
            continue;
        }

        create_tests(&entry.path(), &mut tests);
    }

    let args: Vec<_> = env::args().collect();
    rustc_test::test_main(&args, tests);
}

fn create_tests(path: &Path, list: &mut Vec<TestDescAndFn>) {
    let f = fs::File::open(path).unwrap();
    let tests: tst::Tests = serde_json::from_reader(f).unwrap_or_else(|e|{
        panic!("error: {:?} in {:?}", e, path);
    });

    for test in tests.tests {
        // if test.description == "Tag state Error :" {
            list.push(create_test(test, path));
        // }
    }
}

fn create_test(data: tst::TestData, path: &Path) -> TestDescAndFn {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let name = format!("'{}' in {}", data.description, file_name);

    TestDescAndFn {
        desc: TestDesc::new(DynTestName(name)),
        testfn: DynTestFn(Box::new(move || actual_test(data.clone()))),
    }
}

fn actual_test(data: tst::TestData) {
    let mut parser = xml::Tokenizer::from_str(&data.input);

    for exp_token in &data.output {
        let token = parser.next().unwrap();
        cmp_tokens(token, exp_token);
    }

    if let Some(res) = parser.next() {
        panic!("unexpected token: {:?}", res);
    }
}

fn cmp_tokens(xml_token: Result<xml::Token, xml::Error>, tst_token: &tst::Token) {
    if let &tst::Token::Error(ref tst_err) = tst_token {
        match xml_token {
            Err(ref e) => {
                assert_eq_text!(tst_err, &e.display_chain().to_string());
                return;
            }
            Ok(ref t) => {
                panic!("should have an error, not {:?}", t);
            }
        }
    }

    let xml_token = &xml_token.unwrap();

    match (xml_token, tst_token) {
        (&xml::Token::Comment(ref data1), &tst::Token::Comment(ref data2)) => {
            assert_eq!(data1.to_str(), data2, "comment mismatch");
        }
        (&xml::Token::ElementStart(ref prefix1, ref tag_name1),
         &tst::Token::ElementStart(ref tag_name2)) => {
            let (prefix2, tag_name2) = split_ns(&tag_name2);
            assert_eq!(prefix1.to_str(), prefix2, "tag name prefix mismatch");
            assert_eq!(tag_name1.to_str(), tag_name2, "tag name mismatch");
        }
        (&xml::Token::Attribute((ref prefix1, ref name1), ref value1),
         &tst::Token::Attribute(ref name2, ref value2)) => {
            let (prefix2, name2) = split_ns(&name2);
            assert_eq!(prefix1.to_str(), prefix2, "attribute prefix mismatch");
            assert_eq!(name1.to_str(), name2, "attribute name mismatch");
            assert_eq!(value1.to_str(), value2, "attribute value mismatch");
        }
        (&xml::Token::ElementEnd(ref end1), &tst::Token::ElementEnd(ref end2)) => {
            match (*end1, end2) {
                (xml::ElementEnd::Open, &tst::ElementEnd::Open) => {},
                (xml::ElementEnd::Empty, &tst::ElementEnd::Empty) => {},
                (xml::ElementEnd::Close(ref prefix1, ref name1), &tst::ElementEnd::Close(ref name2)) => {
                    let (prefix2, name2) = split_ns(&name2);
                    assert_eq!(prefix1.to_str(), prefix2);
                    assert_eq!(name1.to_str(), name2);
                },
                _ => {
                    panic!("element end mismatch: {:?} {:?}", end1, end2);
                }
            }
        }
        (&xml::Token::ProcessingInstruction(target1, content1),
         &tst::Token::PI(ref target2, ref content2)) => {
            assert_eq!(target1.to_str(), target2, "PI target mismatch");

            let content1 = content1.map(|s| s.to_str().to_owned());
            assert_eq!(&content1, content2, "PI content mismatch");
        }
        (&xml::Token::Declaration(version1, encoding1, standalone1),
         &tst::Token::Declaration(ref version2, ref encoding2, ref standalone2)) => {
            let encoding1 = encoding1.map(|s| s.to_str().to_owned());
            let standalone1 = standalone1.map(|s| s.to_str().to_owned());

            assert_eq!(version1.to_str(), version2, "declaration version mismatch");
            assert_eq!(&encoding1, encoding2, "declaration encoding mismatch");
            assert_eq!(&standalone1, standalone2, "declaration standalone mismatch");
        }
        (&xml::Token::Text(ref text1), &tst::Token::Text(ref text2)) => {
            assert_eq!(text1.to_str(), text2, "text1 mismatch");
        }
        (&xml::Token::Whitespaces(ref text1), &tst::Token::Whitespaces(ref text2)) => {
            assert_eq!(text1.to_str(), text2, "text1 mismatch");
        }
        (&xml::Token::Cdata(ref text1), &tst::Token::CDATA(ref text2)) => {
            assert_eq!(text1.to_str(), text2, "CDATA mismatch");
        }
          (&xml::Token::EmptyDtd(name1, ref id1),
           &tst::Token::EmptyDTD(ref name2, ref type2, ref literal1_2, ref literal2_2))
        | (&xml::Token::DtdStart(name1, ref id1),
           &tst::Token::DTDStart(ref name2, ref type2, ref literal1_2, ref literal2_2)) => {
            assert_eq!(name1.to_str(), name2, "DTD name mismatch");

            if let Some(ref id1) = *id1 {
                match *id1 {
                    xml::ExternalId::System(literal1_1) => {
                        assert_eq!("SYSTEM", type2);
                        assert_eq!(literal1_1.to_str(), literal1_2);
                    }
                    xml::ExternalId::Public(literal1_1, literal2_1) => {
                        assert_eq!("PUBLIC", type2);
                        assert_eq!(literal1_1.to_str(), literal1_2);
                        assert_eq!(literal2_1.to_str(), literal2_2);
                    }
                }
            }
        }
        (&xml::Token::EntityDeclaration(name1, ref def),
         &tst::Token::EntityDecl(ref name2, ref text1_2, ref text2_2, ref text2_3)) => {
            assert_eq!(name1.to_str(), name2, "ENTITY name mismatch");

            match *def {
                xml::EntityDefinition::EntityValue(text1_1) => {
                    assert_eq!(text1_1.to_str(), text1_2);
                }
                xml::EntityDefinition::ExternalId(ref id1) => {
                    match *id1 {
                        xml::ExternalId::System(text1_1) => {
                            assert_eq!("SYSTEM", text1_2);
                            assert_eq!(text1_1.to_str(), text2_2);
                        }
                        xml::ExternalId::Public(text2_1, text3_1) => {
                            assert_eq!("PUBLIC", text1_2);
                            assert_eq!(text2_1.to_str(), text2_2);
                            assert_eq!(text3_1.to_str(), text2_3);
                        }
                    }
                }
            }
        }
        (&xml::Token::DtdEnd, &tst::Token::DTDEnd) => {
            //
        }
        _ => {
            panic!("unexpected token: {:?}, expected {:?}", xml_token, tst_token);
        }
    }
}

fn split_ns(text: &str) -> (&str, &str) {
    match text.chars().position(|c| c == ':') {
        Some(_) => {
            let mut iter = text.split(':');
            (iter.next().unwrap(), iter.next().unwrap())
        },
        None => ("", text),
    }
}

#[test]
fn bom_1() {
    let mut s = Vec::new();
    s.push(0xEF);
    s.push(0xBB);
    s.push(0xBF);

    let t = str::from_utf8(&s).unwrap();

    let mut p = xml::Tokenizer::from_str(t);
    assert_eq!(p.next().unwrap().unwrap_err().display_chain().to_string(),
               "Error: unexpected end of stream\n");
}
