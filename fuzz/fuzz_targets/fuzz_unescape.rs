#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate xmlparser;

use std::str;

use xmlparser::{TextUnescape, XmlSpace};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let _ = TextUnescape::unescape(s, XmlSpace::Default);
        let _ = TextUnescape::unescape(s, XmlSpace::Preserve);
    }
});
