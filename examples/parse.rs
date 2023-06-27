extern crate htmlparser as html;

use std::io::Read;
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: parse file.html");
        return;
    }

    let text = load_file(&args[1]);

    if let Err(e) = parse(&text) {
        println!("Error: {}.", e);
    }
}

fn parse(text: &str) -> Result<(), html::Error> {
    for token in html::Tokenizer::from(text) {
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
