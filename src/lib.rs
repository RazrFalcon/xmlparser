/*!
A low-level [XML 1.0](https://www.w3.org/TR/xml/) parser implementation.

```rust
use xmlparser::FromSpan;

for token in xmlparser::Tokenizer::from_str("<tagname name='value'/>") {
    println!("{:?}", token);
}
```
*/

#![doc(html_root_url = "https://docs.rs/xmlparser/0.1.0")]

#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;


mod error;
mod stream;
mod strspan;
mod text;
mod token;
mod xml;
mod xmlchar;


pub use error::{
    Error,
    ErrorKind,
    ErrorPos,
};
pub use stream::{
    Reference,
    Stream,
    StreamError,
    StreamErrorKind,
};
pub use text::{
    TextUnescape,
    XmlSpace,
};
pub use strspan::{
    StrSpan,
    FromSpan,
};
pub use token::{
    ElementEnd,
    EntityDefinition,
    ExternalId,
    Token,
};
pub use xml::{
    Tokenizer,
};
pub use xmlchar::{
    XmlByteExt,
    XmlCharExt,
};

pub use error_chain::ChainedError;
