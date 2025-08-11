# `ofx-parse`

A Rust-based parser for the Open Financial Exchange (OFX) file format.

Since it is designed only for reading OFX, not for writing it, it is overly permissive where doing so simplifies the implementation; for example, if a v1.6 field is present in a v1.0.2 document, the parse will succeed.

## Example

```rust
use std::fs;
use ofx_parse::{error::Result, from_str, ofx::header::Ofx};

// ...

let ofx_file = fs::read_to_string(file_path).unwrap();
let ofx: Ofx = ofx_parse::from_str(ofx_file).unwrap();
if let Some(sonrs) = ofx.sonrs {
    // ...
}
```

## Implementatation of OFX Specifications

- [ ] OFX Version `1.0` to `1.6`
  - [ ] Character encodings
- [ ] OFX Version `2.0` to `2.2.1`
  - TODO
