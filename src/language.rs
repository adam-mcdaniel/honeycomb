/// This module is useful for consuming common language
/// tokens such as strings, identifiers, punctuation,
/// floats, and arrays

/// Import necessary atoms
use crate::{
    atoms::{if_take, list, none_of, one_of, opt, seq, seq_no_ws, space, sym},
    transform::{collect, to_string},
    Parser,
};

use alloc::borrow::ToOwned;
use alloc::string::String;
/// We need alloc!
use alloc::vec::Vec;

/// Consumes an alphabetic character
pub fn alpha() -> Parser<char> {
    if_take(|ch| ch.is_ascii_alphabetic())
        % "a letter"
}

/// Consumes a numeric character
pub fn numeral() -> Parser<char> {
    if_take(|ch| ch.is_numeric())
        % "a digit"
}

/// Consumes an alphanumeric character
pub fn alphanumeric() -> Parser<char> {
    alpha() | numeral()
        % "an alphanumeric character"
}

/// Consumes a punctuation character
/// One of ! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~
pub fn punctuation() -> Parser<char> {
    if_take(|ch| ch.is_ascii_punctuation())
        % "one of ! \" # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \\ ] ^ _ ` { | } ~"
}

/// Consumes a common language token like
/// A string
/// A number
/// An identifier
/// Or Punctuation
pub fn token() -> Parser<String> {
    (space()
        >> (string()       // Consume string
        | number()     // Consume number
        | identifier() // Consume identifier
        | (punctuation() - to_string))
        << space())
        % "pne of string, number, identifier, or punctuation"
}

/// Consumes an alphanumeric identifier
pub fn identifier() -> Parser<String> {
    (alpha().is() >> (((alphanumeric() | sym('_')) * (1..31)) - collect))
        % "an identifier"
}

/// Consumes a quoted string
pub fn string() -> Parser<String> {
    let special_char = sym('\\')
        | sym('/')
        | sym('"')
        | sym('\'')
        | (sym('b') - |_| '\x08')
        | (sym('f') - |_| '\x0C')
        | (sym('n') - |_| '\n')
        | (sym('r') - |_| '\r')
        | (sym('t') - |_| '\t');
    let escape_sequence = sym('\\') >> special_char;

    ((sym('"') >> ((none_of(b"\\\"") | escape_sequence).repeat(0..)) << sym('"'))
        - |v| v.iter().collect::<String>())
        % "a string"
}

/// Consumes a number
pub fn number() -> Parser<String> {
    let integer =
        (one_of(b"0123456789").repeat(1..) - |cons| cons.iter().collect::<String>()) | seq("0");

    let frac = sym('.') >> integer.clone();
    let number = (space() >> opt(sym('-'))) & (space() >> integer) & (opt(frac) << space());

    (number
        - |v: ((Option<char>, String), Option<String>)| {
            let mut result = String::new();

            // Add the sign component
            if let Some(ch) = (v.0).0 {
                result.push(ch);
            }

            // Add the integer component
            result += &(v.0).1;

            // If the fractional component exists,
            // append it to the result
            if let Some(s) = v.1 {
                result += &(".".to_owned() + &s);
            }

            result
        })
        % "a number"
}

/// Consumes an array of items
pub fn array<T: 'static + Clone>(
    begin: &'static str,
    item: Parser<T>,
    end: &'static str,
) -> Parser<Vec<T>> {
    seq_no_ws(begin) >> list(item.clone(), seq_no_ws(",")) << seq_no_ws(end)
        % format!("An array of 0 or more {}(s)", item.expectation)
}
