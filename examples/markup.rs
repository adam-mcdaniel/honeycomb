extern crate honeycomb;
use honeycomb::{
    atoms::{rec, space, list, seq_no_ws},
    language::{token, identifier},
    Parser
};

use std::collections::HashMap;


#[derive(Clone, PartialEq, Debug)]
pub enum Markup {
    Value(String),
    Object(HashMap<String, Self>),
    Array(Vec<Self>)
}

fn object() -> Parser<Markup> {
    space() >> (((space() >> identifier() & (seq_no_ws("{") >> markup() << (seq_no_ws("}") << space()))) * (1..))
        - |obj: Vec<(String, Markup)>| {
            let mut map = HashMap::new();
            for (s, y) in obj {
                map.insert(s, y);
            }
            Markup::Object(map)
        }) << space()
}

fn array() -> Parser<Markup> {
    seq_no_ws("-") >> list(rec(markup), seq_no_ws("-"))
        - Markup::Array
}

fn value() -> Parser<Markup> {
    (space() >> token() << space())
        - Markup::Value
}

fn markup() -> Parser<Markup> {
    rec(object)
    | rec(array)
    | rec(value)
}

fn main() {
    println!("{:#?}", (markup() * (1..)).parse(r#"

row {
    column1 {
        width { 5 }
        height { 10 }
    }

    column2 {
        width { 5 }
        height { 10 }
    }

    column3 {
        width { 5 }
        height { 10 }
    }

    columnlist {
        - 1
        - 2
        - 3
    }
}

"#));
}