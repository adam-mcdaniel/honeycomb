use alloc::string::{String, ToString};
/// This module is useful for transforming the output of a parser
/// into something useful. An example of this is converting a
/// Vec<char> into a String.

/// We need alloc!
use alloc::vec::Vec;

/// Converts a Vec<char> to a String
pub fn collect(v: Vec<char>) -> String {
    v.iter().collect::<String>()
}

/// Converts a ToString to a String
pub fn to_string(t: impl ToString) -> String {
    t.to_string()
}

/// Converts a ToString to a f64
pub fn to_number(t: impl ToString) -> f64 {
    match t.to_string().parse::<f64>() {
        Ok(n) => n,
        Err(_) => 0.0,
    }
}
