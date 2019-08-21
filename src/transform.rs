use alloc::string::{String, ToString};
/// This module is useful for transforming the output of a parser
/// into something useful. An example of this is converting a
/// Vec<char> into a String.

/// We need alloc!
use alloc::vec::Vec;
use core::str::FromStr;

/// Converts a Vec<char> to a String
pub fn collect(v: Vec<char>) -> String {
    v.iter().collect::<String>()
}

/// Converts a ToString to a String
pub fn to_string(t: impl ToString) -> String {
    t.to_string()
}

/// Converts a ToString to a f64
pub fn to_number<T>(t: impl ToString) -> T where T: Default + FromStr {
    match t.to_string().parse::<T>() {
        Ok(n) => n,
        Err(_) => Default::default(),
    }
}

/// Unwrap an opt where the type has a default value
pub fn unwrap_opt<T>(t: Option<T>) -> T
where
    T: Default,
{
    match t {
        Some(v) => v,
        None => Default::default(),
    }
}
