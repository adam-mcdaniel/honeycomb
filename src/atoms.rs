use crate::{Error, Parser};
use core::cmp::min;

use alloc::string::{String, ToString};
/// We need alloc!
use alloc::vec::Vec;

/// Consumes a character if a function is true
pub fn if_take(if_fn: fn(char) -> bool) -> Parser<char> {
    Parser::new(
        move |s: &str| {
            let result_ch;
            if let Some(ch) = s.chars().nth(0) {
                if if_fn(ch) {
                    return Ok((ch, s[1..].to_string()));
                }
                result_ch = ch;
            } else {
                result_ch = '\0';
            }

            // Control flow prevents reaching this line unless
            // 1) EOF in string
            // 2) if_fn returned false
            // In these cases, we throw an Error.
            Error::new(
                result_ch.to_string(),
                "result of if_take input".to_string(),
                s.to_string(),
            )
        },
        "result of if_take input".to_string(),
    )
}

/// Consumes a matching character
pub fn sym(symbol: char) -> Parser<char> {
    Parser::new(
        move |s: &str| {
            // If symbol is == s[0], return symbol
            // Otherwise, return Error
            if Some(symbol) == s.chars().nth(0) {
                Ok((symbol, s[1..].to_string()))
            } else {
                let actual = match s.chars().nth(0) {
                    Some(ch) => ch,
                    None => '\0',
                };
                Error::new(actual, symbol, s)
            }
        },
        symbol.to_string(),
    )
}

/// Consumes a matching sequence of characters
pub fn seq(sequence: &'static str) -> Parser<String> {
    Parser::new(
        move |s: &str| {
            // If every character of sequence is accounted for,
            // consume sequence!
            // Otherwise, return Error
            let mut n = 0;
            for ch in sequence.chars() {
                if s.chars().nth(n) != Some(ch) {
                    return Error::new(&s[..min(sequence.len(), s.len())], sequence, s);
                }
                n += 1;
            }
            Ok((s[..n].to_string(), s[n..].to_string()))
        },
        sequence.to_string(),
    )
}

/// Consumes a sequence of characters ignoring preceeding and succeeding whitespace
pub fn seq_no_ws(sequence: &'static str) -> Parser<String> {
    space() >> seq(sequence) << space()
}

/// Succeeds whether or not the parser consumes input
pub fn opt<T: 'static + Clone>(parser: Parser<T>) -> Parser<Option<T>> {
    let expectation = format!("Optionally {}", parser.expectation.clone());
    Parser::new(
        move |s: &str| match parser.parse_internal(s) {
            // Return okay either way!
            Ok((consumed, remaining)) => Ok((Some(consumed), remaining)),
            Err(_) => Ok((None, s.to_string())),
        },
        expectation,
    )
}

/// Consumes any character
pub fn any() -> Parser<char> {
    Parser::new(
        move |s: &str| {
            if let Some(c) = s.chars().nth(0) {
                Ok((c, s[1..].to_string()))
            } else {
                Error::new('\0', "any character", s)
            }
        },
        "any character",
    )
}

/// Consumes any of a list of bytes
pub fn one_of(options: &'static [u8]) -> Parser<char> {
    Parser::new(
        move |s: &str| match s.chars().nth(0) {
            Some(ch) => {
                if options.contains(&(ch as u8)) {
                    Ok((ch, s[1..].to_string()))
                } else {
                    Error::new(ch, format!("One of {:?}", options), s)
                }
            }
            None => Error::new('\0', format!("One of {:?}", options), s),
        },
        format!("one of {:?}", options.iter().map(|n| n.clone() as char).collect::<Vec<char>>()),
    )
}

/// Consumes anything not in a list of bytes
pub fn none_of(options: &'static [u8]) -> Parser<char> {
    Parser::new(
        move |s: &str| match s.chars().nth(0) {
            Some(ch) => {
                if options.contains(&(ch as u8)) {
                    Error::new(ch, format!("None of {:?}", options), s)
                } else {
                    Ok((ch, s[1..].to_string()))
                }
            }
            None => Error::new('\0', format!("None of {:?}", options), s),
        },
        format!("none of {:?}", options.iter().map(|n| n.clone() as char).collect::<Vec<char>>()),
    )
}

/// Consumes nothing, but fails if this parser succeeds
pub fn not<T>(parser: Parser<T>) -> Parser<()>
where
    T: 'static + Clone,
{
    !parser
}

/// Consumes nothing, but succeeds if this parser succeeds
pub fn is<T>(parser: Parser<T>) -> Parser<()>
where
    T: 'static + Clone,
{
    parser.is()
}

/// Consumes whitespace
pub fn space() -> Parser<String> {
    one_of(b" \t\r\n").repeat(0..).map(|v| v.iter().collect())
        % "whitespace"
}

/// Consumes EOF
pub fn eof() -> Parser<()> {
    space().prefixes(Parser::new(
        move |s: &str| match s.chars().nth(0) {
            Some('\0') => Ok(((), s.to_string())),
            Some(ch) => Error::new(ch, "EOF", s),
            None => Ok(((), s.to_string())),
        },
        "EOF",
    )) % "EOF"
}

/// Consumes a list of items separated by a seperating parser
/// This will match the following.
/// A, B, ...
/// A, B,
/// A, B
/// A,
/// A
/// The separating parser will not consume input.
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
    Parser::new(
        move |s| parser().parse_internal(s),
        "result from recursive Parser",
    )
}
