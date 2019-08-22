extern crate honeycomb;
use honeycomb::{
    atoms::{eof, rec, seq, space, sym},
    language::number,
    transform::to_number,
    Parser,
};

use std::io::{stdin, stdout, Write};
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

fn input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);

    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}

fn main() {
    loop {
        let output = input(">>> ");

        match math().parse(&output) {
            Ok(m) => println!("{:#?}\n\nResult: {}", m.clone(), eval(m)),
            Err(_) => println!("Invalid math expression!"),
        }
    }
}
