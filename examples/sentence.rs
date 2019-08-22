extern crate honeycomb;
use honeycomb::{
    atoms::{sym, seq_no_ws, space, opt},
    language::alpha,
    transform::collect,
    Parser
};


#[derive(Clone, PartialEq, Debug)]
struct Word(String);

#[derive(Clone, PartialEq, Debug)]
enum Punctuation {
    Period,
    QuestionMark,
    ExclamationPoint,
}

#[derive(Clone, PartialEq, Debug)]
struct Sentence {
    words: Vec<Word>,
    punctuation: Punctuation
}


fn word() -> Parser<Word> {
    (space() >> (alpha() * (1..)) << space())
        - collect - Word
}

fn punctuation() -> Parser<Punctuation> {
    ((sym('.') * (1..)) - |_| Punctuation::Period)
    | ((sym('?') * (1..)) - |_| Punctuation::QuestionMark)
    | ((sym('!') * (1..)) - |_| Punctuation::ExclamationPoint)
}

fn sentence() -> Parser<Sentence> {
    (word().is() >> (((word() << opt(seq_no_ws(","))) * (1..)) & punctuation()))
        - |phrase: (Vec<Word>, Punctuation)| {
            Sentence {
                words: phrase.0,
                punctuation: phrase.1
            }
        }
}

fn paragraph() -> Parser<Vec<Sentence>> {
    sentence() * (1..)
}


fn main() {
    println!("{:#?}", paragraph().parse(r#"

I look at you all see the love there thats sleeping,
While my guitar gently weeps.
I look at the floor and I see it needs sweeping
Still my guitar gently weeps.
I dont know why nobody told you
How to unfold your love.
I dont know how someone controlled you.
They bought and sold you.
I look at the world and I notice its turning
While my guitar gently weeps.
With every mistake we must surely be learning,
Still my guitar gently weeps.
"#));
}