extern crate honeycomb;
use honeycomb::{
    atoms::{opt, seq_no_ws, space, sym},
    language::{number, punctuation, string},
    transform::{collect, to_btree, to_number, to_string, unwrap_opt},
};

use std::collections::BTreeMap;

#[test]
fn collect_test() {
    assert_eq!(
        ((sym('a') * (..)) - collect).parse("aaaahey jude"),
        Ok(String::from("aaaa"))
    )
}

#[test]
fn to_string_test() {
    assert_eq!(
        (punctuation() - to_string).parse("(aaaahey jude"),
        Ok(String::from("("))
    )
}

#[test]
fn to_number_test() {
    assert_eq!(
        (number() - to_number).parse("19871293.19823"),
        Ok(19871293.19823)
    );

    assert_eq!(
        (number() - to_number).parse(" - 19871293.19823"),
        Ok(-19871293.19823)
    );
}

#[test]
fn to_btree_test() {
    let key_value =
        ((space() >> ((string() << seq_no_ws(":")) & number()) << space()) * (..)) - to_btree;

    assert_eq!(key_value.parse(r#""#), Ok(BTreeMap::new()));

    let mut map = BTreeMap::new();
    map.insert(String::from("testing"), String::from("12345"));
    map.insert(String::from("adam"), String::from("2019"));
    map.insert(String::from("hey yo dude\""), String::from("12312323212"));
    assert_eq!(
        key_value.parse(
            r#"
"testing" : 12345
"adam" : 2019


"hey yo dude\"" : 12312323212
"#
        ),
        Ok(map)
    );
}

#[test]
fn unwrap_opt_test() {
    assert_eq!(
        (opt(string()) - unwrap_opt).parse("1298712.9"),
        Ok(String::from(""))
    );
    assert_eq!(
        (opt(string()) - unwrap_opt).parse("\"1298712.9\""),
        Ok(String::from("1298712.9"))
    );
}
