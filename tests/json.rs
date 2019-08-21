extern crate comb;
use comb::*;

use std::collections::HashMap;
use std::str::{self, FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn token(to_match: &'static str) -> Parser<String> {
    space() >> seq(to_match) << space()
}

fn boolean() -> Parser<JsonValue> {
    token("true") - (|_| JsonValue::Bool(true)) | token("false") - (|_| JsonValue::Bool(false))
}

fn string() -> Parser<JsonValue> {
    let not_escape = not(seq("\\\"")) + sym('"');
    (space() >> token("\"") >> take_until(not_escape) << space()) - |s| JsonValue::Str(s)
}

fn number() -> Parser<JsonValue> {
    space()
        >> ((one_of(b"0123456789-.") * (1..)).map(|v| v.iter().collect::<String>())
            - |s| {
                JsonValue::Num(match s.parse::<f64>() {
                    Ok(n) => n,
                    _ => 0.0,
                })
            })
        << space()
}

fn null() -> Parser<JsonValue> {
    token("null") - |_| JsonValue::Null
}

#[test]
fn json_parser() {
    assert_eq!(
        string().parse("\"testing\"").unwrap(),
        JsonValue::Str(String::from("testing"))
    )
}
