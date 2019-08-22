extern crate honeycomb;
use honeycomb::{
    language::{array, identifier, number, punctuation, string, token},
    Error,
};

#[test]
fn token_test() {
    assert_eq!(
        (token() * (..)).parse(
            r#"

fn main() {
    println!("Hello world!");
}

"#
        ),
        Ok(vec![
            String::from("fn"),
            String::from("main"),
            String::from("("),
            String::from(")"),
            String::from("{"),
            String::from("println"),
            String::from("!"),
            String::from("("),
            String::from("Hello world!"),
            String::from(")"),
            String::from(";"),
            String::from("}"),
        ])
    );
}

#[test]
fn punctuation_test() {
    assert_eq!(punctuation().parse("("), Ok('('));
    assert_eq!(punctuation().parse(")"), Ok(')'));
    assert_eq!(punctuation().parse("}"), Ok('}'));
    assert_eq!(punctuation().parse("{"), Ok('{'));
    assert_eq!(punctuation().parse("["), Ok('['));
    assert_eq!(punctuation().parse("]"), Ok(']'));

    assert_eq!(punctuation().parse("*"), Ok('*'));
    assert_eq!(punctuation().parse("+"), Ok('+'));
    assert_eq!(punctuation().parse("-"), Ok('-'));
    assert_eq!(punctuation().parse("/"), Ok('/'));
    assert_eq!(punctuation().parse(","), Ok(','));
    assert_eq!(punctuation().parse("?"), Ok('?'));
    assert_eq!(punctuation().parse("."), Ok('.'));

    assert_eq!(
        punctuation().parse("a"),
        Error::new("a", "Result of if_take input", "a")
    );
}

#[test]
fn identifier_test() {
    assert_eq!(identifier().parse("testing"), Ok(String::from("testing")));
    assert_eq!(identifier().parse("testing1"), Ok(String::from("testing1")));
    assert_eq!(
        identifier().parse("123testing"),
        Error::new("123testing", "Not 123testing", "123testing")
    );
    assert_eq!(
        identifier().parse("testing13412341234"),
        Ok(String::from("testing13412341234"))
    );
    assert_eq!(
        identifier().parse("testin{}{}{g13412341234"),
        Ok(String::from("testin"))
    );
}

#[test]
fn number_test() {
    assert_eq!(number().parse(" 12312.01  "), Ok(String::from("12312.01")));
    assert_eq!(
        number().parse(" - 12312.01  "),
        Ok(String::from("-12312.01"))
    );

    assert_eq!(number().parse(" - 1  "), Ok(String::from("-1")));
    assert_eq!(number().parse(" - 0  "), Ok(String::from("-0")));
    assert_eq!(number().parse("  0  "), Ok(String::from("0")));
    assert_eq!(number().parse("  01  "), Ok(String::from("01")));
    assert_eq!(number().parse("  1.0112  "), Ok(String::from("1.0112")));
    assert_eq!(number().parse("  1.  "), Ok(String::from("1")));



    let number_convert = number() ^ |n| n.parse::<i32>();

    assert_eq!(
        number_convert.parse("123"),
        Ok(123)
    );

    assert_eq!(
        number_convert.parse("123"),
        Ok(123)
    );

    assert_eq!(
        number_convert.parse("12.33"),
        Error::new("12.33", "A convertible value", "12.33")
    );
}

#[test]
fn array_test() {
    assert_eq!(
        array("[", number(), "]").parse("[1, 3, 5.7, 8]"),
        Ok(vec![
            String::from("1"),
            String::from("3"),
            String::from("5.7"),
            String::from("8"),
        ])
    );

    assert_eq!(
        array("[", string(), "]").parse(
            r#"
[
    "hey jude",
    "don\'t make it \"bad\"",
    "\ttake a sad song",
    "and make it\n \"better\"",
    "\\reg\\ex",
]
"#
        ),
        Ok(vec![
            String::from("hey jude"),
            String::from("don\'t make it \"bad\""),
            String::from("\ttake a sad song"),
            String::from("and make it\n \"better\""),
            String::from("\\reg\\ex")
        ])
    );
}

#[test]
fn string_test() {
    assert_eq!(string().parse("\"hey jude\""), Ok(String::from("hey jude")));

    assert_eq!(
        string().parse("\"don\'t make it \\\"bad\\\"\""),
        Ok(String::from("don\'t make it \"bad\""))
    );

    assert_eq!(
        string().parse("\"\\ttake a sad song\""),
        Ok(String::from("\ttake a sad song"))
    );

    assert_eq!(
        string().parse("\"and make it\\n \\\"better\\\"\""),
        Ok(String::from("and make it\n \"better\""))
    );
}
