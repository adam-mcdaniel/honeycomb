extern crate honeycomb;
use honeycomb::{atoms::rec, language, language::token_is, transform::to_number, Parser};

use std::collections::HashMap;

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

    let mut map = HashMap::new();
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
    Object(HashMap<String, JsonValue>),
}

fn boolean() -> Parser<JsonValue> {
    (token_is("true") - |_| JsonValue::Bool(true))
        | (token_is("false") - |_| JsonValue::Bool(false))
}

fn string() -> Parser<JsonValue> {
    language::string() - JsonValue::Str
}

fn number() -> Parser<JsonValue> {
    language::number() - to_number - JsonValue::Num
}

fn null() -> Parser<JsonValue> {
    token_is("null") - |_| JsonValue::Null
}

fn array() -> Parser<JsonValue> {
    language::array("[", json(), "]") - JsonValue::Array
}

fn object() -> Parser<JsonValue> {
    language::array("{", string() << token_is(":") & rec(json), "}")
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
