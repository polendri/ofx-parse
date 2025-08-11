//! Parser for the Open Financial Exchange (OFX) file format.

// TODO: Determine public interface

mod de;
pub mod error;
pub mod ofx;
mod parse;

pub use de::from_str;
