extern crate comb;
use comb::atoms::seq;

#[test]
fn seq_test() {
    assert_eq!(seq("test").parse("testing"), Ok("test".to_string()));
}
