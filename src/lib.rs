#![no_std]

#[macro_use]
extern crate alloc;

mod parser;
pub use parser::*;

pub mod atoms;
pub mod language;
