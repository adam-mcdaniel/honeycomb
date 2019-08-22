
extern crate honeycomb;
use honeycomb::{
    atoms::{rec, seq_no_ws},
    language,
    transform::{to_number, to_btree},
    Parser,
};

use std::collections::BTreeMap;


#[test]
fn json_test() {
    assert_eq!(
        string().parse("\"test\\\"ing\"").unwrap(),
        String::from("test\"ing")
    );

    assert_eq!(
        number().parse("119871.9193").unwrap(),
        JsonValue::Num(119871.9193)
    );

    let mut map = BTreeMap::new();
    map.insert(String::from("hey jude!"), JsonValue::Num(5.0));

    assert_eq!(
        json()
            .parse(
                r#"
[
    "testing",
    {
        "hey jude!": 5.0
    },
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
            JsonValue::Object(map),
            JsonValue::Num(1.2),
            JsonValue::Null,
            JsonValue::Bool(true),
            JsonValue::Bool(false)
        ])
    );
}

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