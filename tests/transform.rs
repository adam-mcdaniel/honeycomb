extern crate comb;
use comb::{
    atoms::sym,
    language::punctuation,
    transform::{collect, to_string},
};

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
