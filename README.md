## libxmlparser [![Build Status]](https://travis-ci.org/RazrFalcon/libxmlparser)

[Build Status]: https://travis-ci.org/RazrFalcon/libxmlparser.svg?branch=master

*libxmlparser* is a low-level, pull-based, zero-allocation
[XML 1.0](https://www.w3.org/TR/xml/) parser.

## Table of Contents
- [Documentation](#documentation)
- [Example](#example)
- [Why a new library](#why-a-new-library)
- [Benefits](#benefits)
- [Limitations](#limitations)
- [Safety](#safety)
- [Usage](#usage)
- [License](#license)

### [Documentation](https://docs.rs/xmlparser/)

### Example

```rust
extern crate xmlparser;

use xmlparser::FromSpan;

for token in xmlparser::Tokenizer::from_str("<tagname name='value'/>") {
    println!("{:?}", token);
}
```

### Why a new library

The main idea of this library is to provide a fast, low-level and complete XML parser.

Unlike other XML parsers, this one can return tokens not with `&str`/`&[u8]` data, but
with `StrSpan` objects, which contain a position of the data in the original document.
Which can be very useful if you want to post-process tokens even more and want to return
errors with a meaningful position.

So, this is basically an XML parser framework that can be used to write parsers for XML-based formats,
like SVG and to construct a DOM.

At the time of writing the only option was `quick-xml` (v0.10), which does not support DTD and
token positions.

Detailed comparison with other XML parsers can be found at
[choose-your-xml-rs](https://github.com/RazrFalcon/choose-your-xml-rs).

### Benefits
 - Tokens contain `StrSpan` objects, that contains a position of the data in the original document.
 - Supports basic text escaping with `xml:space` (should be invoked manually).
   A properer text escaping is very hard without a DOM construction.
 - Good error processing. All error types contain position (line:column) where it occurred.
 - No heap allocations.

### Limitations
 - Currently, only ENTITY objects are parsed from the DOCTYPE. Other ignored.
 - No namespaces. Still, they are mostly useless without a DOM.
 - UTF-8 only.

### Safety

 - The library should not panic. Any panic considered as a critical bug
   and should be reported.
 - The library forbids unsafe code.

### Usage

Dependency: [Rust](https://www.rust-lang.org/) >= 1.13 (>= 1.15 for testing)

Add this to your `Cargo.toml`:

```toml
[dependencies]
xmlparser = "0.1"
```

### License

*libxmlparser* is licensed under the **MIT**.
