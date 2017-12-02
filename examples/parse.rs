extern crate xmlparser as xml;
extern crate stderrlog;

use std::env;
use std::fs;
use std::io::Read;

use xml::{FromSpan, ChainedError};

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: parse file.xml");
        return;
    }

    let text = load_file(&args[1]);

    if let Err(e) = parse(&text) {
        println!("{}", e.display_chain().to_string());
    }
}

fn parse(text: &str) -> Result<(), xml::Error> {
    for token in xml::Tokenizer::from_str(&text) {
        println!("{:?}", token?);
    }

    Ok(())
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}
