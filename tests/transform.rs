extern crate honeycomb;
use honeycomb::{
    atoms::{sym, opt},
    language::{number, punctuation, string},
    transform::{collect, to_number, to_string, unwrap_opt},
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
