#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate htmlparser;

use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = str::from_utf8(data) {
        let mut n = 0;
        for _ in htmlparser::Tokenizer::from(text) {
            n += 1;

            if n == 1000 {
                panic!("endless loop");
            }
        }
    }
});
