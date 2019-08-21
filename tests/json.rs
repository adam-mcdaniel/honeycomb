extern crate comb;
use comb::*;

use std::collections::HashMap;
use std::str::{self};

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
    let special_char = sym('"');
    let escape_sequence = sym('\\') >> special_char;

    (sym('"') >> ((none_of(b"\\\"") | escape_sequence).repeat(0..)) << sym('"'))
        .map(|v| v.iter().collect::<String>())
        - |s| JsonValue::Str(s.replace("\\\"", "\""))
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

fn array() -> Parser<JsonValue> {
    (token("[") >> list(rec(json), token(",")) << token("]"))
        - |v: Vec<JsonValue>| JsonValue::Array(v)
}

fn object() -> Parser<JsonValue> {
    (token("{") >> list(string() << token(":") & rec(json), token(",")) << token("}"))
        - (|v: Vec<(JsonValue, JsonValue)>| -> JsonValue {
            let mut result = HashMap::new();
            for (key, value) in v {
                match key {
                    JsonValue::Str(s) => {
                        result.insert(s, value);
                    }
                    _ => {}
                }
            }
            JsonValue::Object(result)
        })
}

fn json() -> Parser<JsonValue> {
    null() | boolean() | number() | string() | rec(array) | rec(object)
}

#[test]
fn json_parser() {
    assert_eq!(
        string().parse("\"test\\\"ing\"").unwrap(),
        JsonValue::Str(String::from("test\"ing"))
    );

    assert_eq!(
        number().parse("119871.9193").unwrap(),
        JsonValue::Num(119871.9193)
    );

    assert_eq!(
        json()
            .parse(
                r#"
[
    "testing",
    1.2,
    null,
    true,
    false
]
"#
            )
            .unwrap(),
        JsonValue::Array(vec![
            JsonValue::Str(String::from("testing")),
            JsonValue::Num(1.2),
            JsonValue::Null,
            JsonValue::Bool(true),
            JsonValue::Bool(false)
        ])
    );
}
