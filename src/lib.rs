#![no_std]

#[macro_use]
extern crate alloc;

/// This module contains the Parser and Error types which
/// contain the minimal logic for implementing the atomic
/// parser combinators.
mod parser;
pub use parser::*;

/// This module contains the atoms necessary for writing any parser.
pub mod atoms;

/// This module is useful for consuming common language
/// tokens such as strings, identifiers, punctuation,
/// floats, and arrays.
pub mod language;

/// This module is useful for transforming the output of a parser
/// into something useful. An example of this is converting a
/// Vec<char> into a String.
pub mod transform;
