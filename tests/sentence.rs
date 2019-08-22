extern crate honeycomb;
use honeycomb::{
    atoms::{opt, seq_no_ws, space, sym},
    language::alpha,
    transform::collect,
    Parser,
};

#[test]
fn sentence_test() {
    assert_eq!(
        paragraph().parse(
            r#"
This is a sentence. This is another one!
"#
        ),
        Ok(vec![
            Sentence {
                words: vec![
                    Word(String::from("This")),
                    Word(String::from("is")),
                    Word(String::from("a")),
                    Word(String::from("sentence"))
                ],
                punctuation: Punctuation::Period
            },
            Sentence {
                words: vec![
                    Word(String::from("This")),
                    Word(String::from("is")),
                    Word(String::from("another")),
                    Word(String::from("one"))
                ],
                punctuation: Punctuation::ExclamationPoint
            },
        ])
    );
}

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
    punctuation: Punctuation,
}

fn word() -> Parser<Word> {
    (space() >> (alpha() * (1..)) << space()) - collect - Word
}

fn punctuation() -> Parser<Punctuation> {
    ((sym('.') * (1..)) - |_| Punctuation::Period)
        | ((sym('?') * (1..)) - |_| Punctuation::QuestionMark)
        | ((sym('!') * (1..)) - |_| Punctuation::ExclamationPoint)
}

fn sentence() -> Parser<Sentence> {
    (word().is() >> (((word() << opt(seq_no_ws(","))) * (1..)) & punctuation()))
        - |phrase: (Vec<Word>, Punctuation)| Sentence {
            words: phrase.0,
            punctuation: phrase.1,
        }
}

fn paragraph() -> Parser<Vec<Sentence>> {
    sentence() * (1..)
}
