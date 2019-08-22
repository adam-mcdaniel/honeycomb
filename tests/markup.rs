extern crate honeycomb;
use honeycomb::{
    atoms::{rec, space, list, seq_no_ws},
    language::{token, identifier},
    Parser
};

use std::collections::HashMap;


#[test]
fn markup_test() {
    let mut map = HashMap::new();
    map.insert(
        String::from("row"),
        Markup::Array(
            vec![
                Markup::Value(String::from("1")),
                Markup::Value(String::from("2")),
                Markup::Value(String::from("3"))
            ]
        )
    );

    assert_eq!(
        (markup() * (1..)).parse(r#"
row {
    - 1
    - 2
    - 3
}"#),
        Ok(vec![Markup::Object(map)])
    );

}



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