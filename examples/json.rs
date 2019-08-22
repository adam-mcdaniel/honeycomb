extern crate honeycomb;
use honeycomb::{atoms::{rec, seq_no_ws}, language, transform::to_number, Parser};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn boolean() -> Parser<JsonValue> {
    (seq_no_ws("true") - |_| JsonValue::Bool(true))
        | (seq_no_ws("false") - |_| JsonValue::Bool(false))
}

fn string() -> Parser<JsonValue> {
    language::string() - JsonValue::Str
}

fn number() -> Parser<JsonValue> {
    language::number() - to_number - JsonValue::Num
}

fn null() -> Parser<JsonValue> {
    seq_no_ws("null") - |_| JsonValue::Null
}

fn array() -> Parser<JsonValue> {
    language::array("[", json(), "]") - JsonValue::Array
}

fn object() -> Parser<JsonValue> {
    language::array("{", string() << seq_no_ws(":") & rec(json), "}")
        - (|v: Vec<(JsonValue, JsonValue)>| -> JsonValue {
            let mut result = HashMap::new();
            for (key, value) in v {
                if let JsonValue::Str(s) = key {
                    result.insert(s, value);
                }
            }
            JsonValue::Object(result)
        })
}

fn json() -> Parser<JsonValue> {
    null() | boolean() | number() | string() | rec(array) | rec(object)
}

fn main() {
    println!(
        "{:#?}",
        json().parse(
            r#"
{
    "testing" : null,
    "recursion" : {
        "WOW": 1.2345
    },
    "array": [1, 2, {"test": "123"}, 4],
    "test": "testing"
}
"#
        )
    );
}
