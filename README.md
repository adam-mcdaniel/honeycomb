# honeycomb
A portable parser combinator library that does not require a runtime

## Documentation

You can find documentation for honeycomb [here](https://docs.rs/honeycomb).

## Dependencies

None! Honeycomb doesn't even require the standard library!

All you need is a device that can run Rust, and you're good to go.

## JSON Parser

Here's an example JSON parser.

Essentially, we define functions that create larger Parsers from small atomic parsers. We create Parsers that parse: boolean, string, numberic, and null values, and then use those to build parsers that consume Arrays, and Object definitions.

```rust
extern crate honeycomb;
use honeycomb::{
    atoms::{rec, seq_no_ws},
    language,
    transform::{to_btree, to_number},
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
    (seq_no_ws("true").map(|_| JsonValue::Bool(true)))
        | (seq_no_ws("false").map(|_| JsonValue::Bool(false)))
}

fn string() -> Parser<String> {
    language::string()
}

fn number() -> Parser<JsonValue> {
    language::number()
        .map(to_number)
        .map(JsonValue::Num)
}

fn null() -> Parser<JsonValue> {
    seq_no_ws("null")
        .map(|_| JsonValue::Null)
}

fn array() -> Parser<JsonValue> {
    language::array("[", json(), "]")
        .map(JsonValue::Array)
}

fn object() -> Parser<JsonValue> {
    language::array("{", string().suffix(seq_no_ws(":")) & rec(json), "}")
        .map(to_btree).map(JsonValue::Object)
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
```

## Programming Language Tokenizer

This uses a built in Parser called token.

The token parser returns a vector of strings that represent identifiers, strings, numbers, and symbols.

Here, we take the token parser, and create a parser that accepts any number of tokens.

```rust
extern crate honeycomb;
use honeycomb::language::token;

fn main() {
    println!(
        "{:?}",
        (token().repeat(..)).parse(
            r#"

struct Point {
    x: i32, y: i32
}


fn testing() {
    println("hello world!");
}

fn main() {

}
"#
        )
    );
}

```