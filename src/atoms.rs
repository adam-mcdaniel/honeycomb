use crate::{Error, Parser};
use core::cmp::min;

use alloc::string::{String, ToString};
/// We need alloc!
use alloc::vec::Vec;

/// Consumes a matching character
pub fn sym(symbol: char) -> Parser<char> {
    Parser::new(move |s: &str| {
        if Some(symbol.clone()) == s.chars().nth(0) {
            Ok((symbol, s[1..].to_string()))
        } else {
            let actual = match s.chars().nth(0) {
                Some(ch) => ch,
                None => '\0',
            };
            Error::new(actual, symbol, s)
        }
    })
}

/// Consumes a matching sequence of characters
pub fn seq(symbol: &'static str) -> Parser<String> {
    Parser::new(move |s: &str| {
        let mut n = 0;
        for ch in symbol.chars() {
            if s.chars().nth(n) != Some(ch) {
                return Error::new(&s[..min(symbol.len(), s.len())], symbol, s);
            }
            n += 1;
        }
        Ok((s[..n].to_string(), s[n..].to_string()))
    })
}

/// Consumes any character
pub fn any() -> Parser<char> {
    Parser::new(move |s: &str| {
        if let Some(c) = s.chars().nth(0) {
            Ok((c, s[1..].to_string()))
        } else {
            return Error::new('\0', "Any character", s);
        }
    })
}

/// Consumes any of a list of bytes
pub fn one_of(options: &'static [u8]) -> Parser<char> {
    Parser::new(move |s: &str| match s.chars().nth(0) {
        Some(ch) => {
            if options.contains(&(ch as u8)) {
                Ok((ch, s[1..].to_string()))
            } else {
                return Error::new(ch, format!("One of {:?}", options), s);
            }
        }
        None => Error::new('\0', format!("One of {:?}", options), s),
    })
}

/// Consumes anything not in a list of bytes
pub fn none_of(options: &'static [u8]) -> Parser<char> {
    Parser::new(move |s: &str| match s.chars().nth(0) {
        Some(ch) => {
            if options.contains(&(ch as u8)) {
                Error::new(ch, format!("None of {:?}", options), s)
            } else {
                Ok((ch, s[1..].to_string()))
            }
        }
        None => Error::new('\0', format!("None of {:?}", options), s),
    })
}

/// Consumes nothing, but fails if this parser succeeds
pub fn not<T>(parser: Parser<T>) -> Parser<()>
where
    T: 'static + Clone,
{
    Parser::new(move |s: &str| match parser.parse_internal(s) {
        Ok(_) => Error::new(s, format!("Not {}", s), s),
        Err(_) => Ok(((), s.to_string())),
    })
}

/// Consumes whitespace
pub fn space() -> Parser<String> {
    one_of(b" \t\r\n").repeat(0..).map(|v| v.iter().collect())
}

/// Consumes EOF
pub fn eof() -> Parser<()> {
    space().prefixes(Parser::new(move |s: &str| match s.chars().nth(0) {
        Some('\0') => Ok(((), s.to_string())),
        Some(ch) => Error::new(ch, "EOF", s),
        None => Ok(((), s.to_string())),
    }))
}

/// Consumes a list of items separated by a seperating parser
/// This will match the following.
/// A, B, ...
/// A, B,
/// A, B
/// A,
/// A
/// This parser will also consume no input.
pub fn list<A, B>(parser: Parser<A>, sep: Parser<B>) -> Parser<Vec<A>>
where
    A: 'static + Clone,
    B: 'static + Clone,
{
    parser
        .clone()
        .suffix(sep)
        .repeat(0..)
        .and(parser.repeat(0..1))
        .map(|v: (Vec<A>, Vec<A>)| {
            let mut vec = (v.0).clone();
            if (v.1).is_empty() {
                return vec;
            }
            vec.push((v.clone().1)[0].clone());
            vec
        })
}

/// This allows us to make recursive parsers
pub fn rec<T>(parser: fn() -> Parser<T>) -> Parser<T>
where
    T: 'static + Clone,
{
    Parser::new(move |s| parser().parse_internal(s))
}
