use core::fmt;
use core::ops::Bound::*;
use core::ops::{BitAnd, BitOr, Mul, Not, RangeBounds, Shl, Shr, Sub};
use core::cmp::min;

extern crate alloc;
use alloc::sync::Arc;

#[derive(Clone, PartialEq)]
pub struct Error {
    actual: String,
    expected: String,
    remaining_input: String,
}

impl Error {
    pub fn new<T>(
        actual: impl ToString,
        expected: impl ToString,
        remaining_input: impl ToString,
    ) -> Result<T, Self> {
        Err(Self {
            actual: actual.to_string(),
            expected: expected.to_string(),
            remaining_input: remaining_input.to_string(),
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Expected `{}` but found `{}` when parsing `{}`",
            self.expected,
            self.actual,
            self.remaining_input.replace("\n", "")
        )
    }
}

type Output<T> = Result<(T, String), Error>;
type Combinator<T> = Arc<dyn Fn(&str) -> Output<T>>;

#[derive(Clone)]
pub struct Parser<T> {
    parser: Combinator<T>,
}

impl<T> Parser<T>
where
    T: 'static + Clone,
{
    pub fn parse(&self, input: &str) -> Result<T, Error> {
        match self.parse_internal(input) {
            Ok(t) => Ok(t.0),
            Err(e) => Err(e),
        }
    }

    pub fn parse_internal(&self, input: &str) -> Output<T> {
        (self.parser)(input)
    }

    fn new(parser: impl Fn(&str) -> Output<T> + 'static) -> Self {
        Self {
            parser: Arc::new(parser),
        }
    }

    pub fn map<O>(self, map_fn: fn(T) -> O) -> Parser<O>
    where
        O: 'static + Clone,
    {
        Parser::new(move |s: &str| match self.parse_internal(s) {
            Ok((first_out, input)) => Ok((map_fn(first_out), input)),
            Err(e) => Err(e),
        })
    }

    pub fn prefixes<O>(self, operand: Parser<O>) -> Parser<O>
    where
        O: 'static + Clone,
    {
        Parser::new(move |s: &str| {
            let (_, output) = self.parse_internal(s)?;
            let (second, output) = operand.parse_internal(&output)?;
            Ok((second, output))
        })
    }

    pub fn suffix<O>(self, operand: Parser<O>) -> Parser<T>
    where
        O: 'static + Clone,
    {
        Parser::new(move |s: &str| {
            let (first, output) = self.parse_internal(s)?;
            let (_, output) = operand.parse_internal(&output)?;
            Ok((first, output))
        })
    }

    pub fn and<O>(self, operand: Parser<O>) -> Parser<(T, O)>
    where
        O: 'static + Clone,
    {
        Parser::new(move |s: &str| {
            let (first, output) = self.parse_internal(s)?;
            let (second, output) = operand.parse_internal(&output)?;
            Ok(((first, second), output))
        })
    }

    pub fn or(self, operand: Self) -> Self {
        Parser::new(move |s: &str| match self.parse_internal(s) {
            Ok(t) => Ok(t),
            Err(_) => operand.parse_internal(s),
        })
    }

    pub fn discard(self) -> Parser<()> {
        Parser::new(move |s: &str| match self.parse_internal(s) {
            Ok((_, input)) => Ok(((), input)),
            Err(e) => Err(e),
        })
    }

    pub fn repeat(self, range: impl RangeBounds<usize>) -> Parser<Vec<T>> {
        let end = match range.end_bound() {
            Unbounded => &std::usize::MAX,
            Excluded(n) => n,
            Included(n) => n,
        }
        .clone();

        let start = match range.start_bound() {
            Unbounded => &0,
            Excluded(n) => n,
            Included(n) => n,
        }
        .clone();

        Parser::new(move |s: &str| {
            let mut input = s.to_string();
            let mut accum = vec![];
            for n in 0..end {
                match self.parse_internal(&input) {
                    Ok((out, unconsumed)) => {
                        accum.push(out);
                        input = unconsumed;
                    }
                    Err(e) => {
                        if n < start {
                            return Err(e);
                        } else {
                            return Ok((accum, input));
                        }
                    }
                }
            }
            Ok((accum, input))
        })
    }
}

impl<T: 'static + Clone> BitOr for Parser<T> {
    type Output = Parser<T>;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl<A: 'static + Clone, B: 'static + Clone> BitAnd<Parser<B>> for Parser<A> {
    type Output = Parser<(A, B)>;
    fn bitand(self, rhs: Parser<B>) -> Self::Output {
        self.and(rhs)
    }
}

impl<T: 'static + Clone> Not for Parser<T> {
    type Output = Parser<()>;
    fn not(self) -> Self::Output {
        not(self)
    }
}

impl<A: 'static + Clone, B: 'static + Clone> Shl<Parser<B>> for Parser<A> {
    type Output = Parser<A>;
    fn shl(self, rhs: Parser<B>) -> Self::Output {
        self.suffix(rhs)
    }
}

impl<A: 'static + Clone, B: 'static + Clone> Shr<Parser<B>> for Parser<A> {
    type Output = Parser<B>;
    fn shr(self, rhs: Parser<B>) -> Self::Output {
        self.prefixes(rhs)
    }
}

impl<T: 'static + Clone, R: RangeBounds<usize>> Mul<R> for Parser<T> {
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: R) -> Self::Output {
        self.repeat(rhs)
    }
}

impl<O, T> Sub<fn(T) -> O> for Parser<T>
where
    O: 'static + Clone,
    T: 'static + Clone,
{
    type Output = Parser<O>;
    fn sub(self, rhs: fn(T) -> O) -> Self::Output {
        self.map(rhs)
    }
}

pub fn sym(symbol: char) -> Parser<char> {
    Parser::new(move |s: &str| {
        if Some(symbol.clone()) == s.chars().nth(0) {
            Ok((symbol, s[1..].to_string()))
        } else {
            let actual = match s.chars().nth(0) {
                Some(ch) => ch,
                None => '\0',
            };
            Error::new(actual, symbol, s)
        }
    })
}

pub fn seq(symbol: &'static str) -> Parser<String> {
    Parser::new(move |s: &str| {
        let mut n = 0;
        for ch in symbol.chars() {
            if s.chars().nth(n) != Some(ch) {
                return Error::new(&s[..min(symbol.len(), s.len())], symbol, s);
            }
            n += 1;
        }
        Ok((s[..n].to_string(), s[n..].to_string()))
    })
}

pub fn any() -> Parser<char> {
    Parser::new(move |s: &str| {
        if let Some(c) = s.chars().nth(0) {
            Ok((c, s[1..].to_string()))
        } else {
            return Error::new('\0', "Any character", s);
        }
    })
}

pub fn take_until<T>(parser: Parser<T>) -> Parser<String>
where
    T: 'static + Clone,
{
    Parser::new(move |s: &str| {
        let mut n = 0;
        let input = String::from(s);

        while let Err(_) = parser.clone().parse(&input[0..n]) {
            n += 1;
            if n >= s.len() {
                n -= 1;
                break;
            }
        }

        Ok((input[0..n].to_string(), input[n..].to_string()))
    })
}

pub fn one_of(options: &'static [u8]) -> Parser<char> {
    Parser::new(move |s: &str| match s.chars().nth(0) {
        Some(ch) => {
            if options.contains(&(ch as u8)) {
                Ok((ch, s[1..].to_string()))
            } else {
                return Error::new(ch, format!("One of {:?}", options), s);
            }
        }
        None => Error::new('\0', format!("One of {:?}", options), s),
    })
}

pub fn none_of(options: &'static [u8]) -> Parser<char> {
    Parser::new(move |s: &str| match s.chars().nth(0) {
        Some(ch) => {
            if options.contains(&(ch as u8)) {
                Error::new(ch, format!("None of {:?}", options), s)
            } else {
                Ok((ch, s[1..].to_string()))
            }
        }
        None => Error::new('\0', format!("None of {:?}", options), s),
    })
}

pub fn not<T>(parser: Parser<T>) -> Parser<()>
where
    T: 'static + Clone,
{
    Parser::new(move |s: &str| match parser.parse_internal(s) {
        Ok(_) => Error::new(s, format!("Not {}", s), s),
        Err(_) => Ok(((), s.to_string())),
    })
}

pub fn space() -> Parser<String> {
    one_of(b" \t\r\n").repeat(0..).map(|v| v.iter().collect())
}

pub fn eof() -> Parser<()> {
    space().prefixes(Parser::new(move |s: &str| match s.chars().nth(0) {
        Some('\0') => Ok(((), s.to_string())),
        Some(ch) => Error::new(ch, "EOF", s),
        None => Ok(((), s.to_string())),
    }))
}

pub fn list<A, B>(parser: Parser<A>, sep: Parser<B>) -> Parser<Vec<A>>
where
    A: 'static + Clone,
    B: 'static + Clone,
{
    parser
        .clone()
        .suffix(sep)
        .repeat(0..)
        .and(parser.repeat(0..1))
        .map(|v: (Vec<A>, Vec<A>)| {
            let mut vec = (v.0).clone();
            if (v.1).is_empty() {
                return vec;
            }
            vec.push((v.clone().1)[0].clone());
            vec
        })
}

pub fn rec<T>(parser: fn() -> Parser<T>) -> Parser<T>
where
    T: 'static + Clone,
{
    Parser::new(move |s| parser().parse_internal(s))
}
