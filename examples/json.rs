extern crate honeycomb;
use honeycomb::{
    atoms::{rec, seq_no_ws},
    language,
    transform::{to_number, to_btree},
    Parser,
};

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

fn boolean() -> Parser<JsonValue> {
    (seq_no_ws("true") - |_| JsonValue::Bool(true))
    | (seq_no_ws("false") - |_| JsonValue::Bool(false))
}

fn string() -> Parser<String> {
    language::string()
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
        - to_btree - JsonValue::Object
}

fn json() -> Parser<JsonValue> {
    null() | boolean() | number() | (string() - JsonValue::Str) | rec(array) | rec(object)
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
