extern crate honeycomb;
use honeycomb::{
    atoms::{list, rec, seq_no_ws, space},
    language::{identifier, token},
    transform::to_btree,
    Parser,
};

use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Markup {
    Value(String),
    Object(BTreeMap<String, Self>),
    Array(Vec<Self>),
}

fn object() -> Parser<Markup> {
    space()
        >> (((space() >> identifier()
            & (seq_no_ws("{") >> markup() << (seq_no_ws("}") << space())))
            * (1..))
            - to_btree
            - Markup::Object)
        << space()
}

fn array() -> Parser<Markup> {
    seq_no_ws("-") >> list(rec(markup), seq_no_ws("-")) - Markup::Array
}

fn value() -> Parser<Markup> {
    (space() >> token() << space()) - Markup::Value
}

fn markup() -> Parser<Markup> {
    rec(object) | rec(array) | rec(value)
}

fn main() {
    println!(
        "{:#?}",
        (markup() * (1..)).parse(
            r#"

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

"#
        )
    );
}
