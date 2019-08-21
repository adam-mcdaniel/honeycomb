extern crate honeycomb;
use honeycomb::{
    atoms::{eof, rec, seq, space, sym},
    language::number,
    transform::to_number,
    Parser,
};

#[test]
fn calculator_test() {
    assert_eq!(eval(math().parse("1 + 3 + 4").unwrap()), 8.0);

    assert_eq!(eval(math().parse("4 + (1 + 3)").unwrap()), 8.0);

    assert_eq!(eval(math().parse("(4 + 1 + 3)").unwrap()), 8.0);

    assert_eq!(eval(math().parse("(4 + (1 + 3))").unwrap()), 8.0);

    assert_eq!(eval(math().parse("(4 + (1 * 3))").unwrap()), 7.0);

    assert_eq!(eval(math().parse("(4 + (1 / 2))").unwrap()), 4.5);

    assert_eq!(eval(math().parse("(4 + (1 - 2))").unwrap()), 3.0);

    assert_eq!(eval(math().parse("5").unwrap()), 5.0);
}

use std::sync::Arc;

#[derive(Clone, Debug)]
enum Math {
    Add(Arc<Math>, Arc<Math>),
    Multiply(Arc<Math>, Arc<Math>),
    Divide(Arc<Math>, Arc<Math>),
    Subtract(Arc<Math>, Arc<Math>),
    Number(f64),
    Exit,
    Clear,
    EOF,
}

fn token(symbol: &'static str) -> Parser<String> {
    space() >> seq(symbol) << space()
}

fn operation(symbol: char, map_fn: fn((Math, Math)) -> Math) -> Parser<Math> {
    (number() - to_number - Math::Number | rec(math))
        .suffix(space() & sym(symbol) & space())
        .and(rec(math))
        - map_fn
}

fn add() -> Parser<Math> {
    operation('+', |m| Math::Add(Arc::new(m.0), Arc::new(m.1)))
}

fn multiply() -> Parser<Math> {
    operation('*', |m| Math::Multiply(Arc::new(m.0), Arc::new(m.1)))
}

fn divide() -> Parser<Math> {
    operation('/', |m| Math::Divide(Arc::new(m.0), Arc::new(m.1)))
}

fn subtract() -> Parser<Math> {
    operation('-', |m| Math::Subtract(Arc::new(m.0), Arc::new(m.1)))
}

fn exit() -> Parser<Math> {
    (seq("exit") | seq("quit")) - |_| Math::Exit
}

fn clear() -> Parser<Math> {
    seq("clear") - |_| Math::Clear
}

fn math() -> Parser<Math> {
    exit()
        | eof() - (|_| Math::EOF)
        | clear()
        | token("(") >> rec(math) << token(")")
        | (number().is()
            >> (multiply() | divide() | add() | subtract() | (number() - to_number - Math::Number)))
}

fn eval(math: Math) -> f64 {
    match math {
        Math::Number(n) => n,
        Math::Add(a, b) => eval((*a).clone()) + eval((*b).clone()),
        Math::Subtract(a, b) => eval((*a).clone()) - eval((*b).clone()),
        Math::Divide(a, b) => eval((*a).clone()) / eval((*b).clone()),
        Math::Multiply(a, b) => eval((*a).clone()) * eval((*b).clone()),
        Math::Exit => std::process::exit(0),
        Math::Clear => {
            println!("{}", "\n".repeat(1000));
            0.0
        }
        _ => 0.0,
    }
}
