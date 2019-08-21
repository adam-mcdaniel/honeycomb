extern crate comb;
use comb::*;

#[derive(Debug, Clone, PartialEq)]
struct Number(i32);

impl From<Vec<char>> for Number {
    fn from(vec: Vec<char>) -> Self {
        Number(match vec.iter().collect::<String>().parse::<i32>() {
            Ok(t) => t,
            Err(_) => 0,
        })
    }
}

impl From<String> for Number {
    fn from(string: String) -> Self {
        Number(match string.parse::<i32>() {
            Ok(t) => t,
            Err(_) => 0,
        })
    }
}

#[test]
fn map_test() {
    assert_eq!(
        (one_of(b"1234567890").repeat(..).map(Number::from))
            .parse("1234")
            .unwrap(),
        (one_of(b"1234567890") * (..) - Number::from)
            .parse("1234")
            .unwrap(),
    );

    assert_eq!(
        (one_of(b"1234567890").repeat(..).map(Number::from))
            .parse("1234")
            .unwrap(),
        Number(1234)
    );
}
