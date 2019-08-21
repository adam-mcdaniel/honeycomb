extern crate comb;
use comb::*;

#[test]
fn any_test() {
    assert_eq!(
        ((any() * (..)) - |v: Vec<char>| v.iter().collect::<String>())
            .parse("asdfaksdjhfaksjd{}{}(*&!*&@%&h 12309\n \r\t")
            .unwrap(),
        String::from("asdfaksdjhfaksjd{}{}(*&!*&@%&h 12309\n \r\t")
    );
}
