extern crate honeycomb;
use honeycomb::{
    atoms::{any, eof, if_take, none_of, seq, sym},
    Error,
};

#[test]
fn is_test() {
    assert_eq!(sym('a').is().parse("aaa"), Ok(()));

    assert_eq!(
        sym('t').isnt().parse("test"),
        Error::new("test", "Not t", "test")
    );

    assert_eq!(sym('t').isnt().parse("hey"), Ok(()));

    assert_eq!(
        sym('t').is().parse("hey"),
        Error::new("hey", "t", "hey")
    );
}

#[test]
fn if_take_test() {
    assert_eq!(
        (if_take(|ch| ch == 'a') * (1..)).parse("aaaaatest"),
        Ok(vec!['a', 'a', 'a', 'a', 'a'])
    );

    // Test EOF
    assert_eq!(
        (if_take(|ch| ch == 'a') * (1..)).parse(""),
        Error::new('\0', "result of if_take input Included(1)..Unbounded times".to_string(), "".to_string())
    );
}

#[test]
fn eof_test() {
    assert_eq!(eof().parse("wow bro"), Error::new("w", "EOF", "wow bro"));
    assert_eq!(eof().parse(""), Ok(()));
}

#[test]
fn none_of_test() {
    assert_eq!(none_of(b"test").parse("wow bro"), Ok('w'));
    assert_eq!(
        none_of(b"test").parse(""),
        Error::new('\0', format!("none of {:?}", "test".chars().collect::<Vec<char>>()), "")
    );
}

#[test]
fn sym_test() {
    assert_eq!(sym('b').parse("btest"), Ok('b'));

    assert_eq!(sym('t').parse(""), Error::new("\0", "t", ""));
}

#[test]
fn seq_test() {
    assert_eq!(seq("test").parse("testing"), Ok("test".to_string()));
}

#[test]
fn any_test() {
    assert_eq!(
        ((any() * (..)) - |v: Vec<char>| v.iter().collect::<String>())
            .parse("asdfaksdjhfaksjd{}{}(*&!*&@%&h 12309\n \r\t")
            .unwrap(),
        String::from("asdfaksdjhfaksjd{}{}(*&!*&@%&h 12309\n \r\t")
    );
}
