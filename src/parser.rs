use core::fmt;
use core::ops::Bound::*;
use core::ops::{BitAnd, BitOr, Mul, Not, RangeBounds, Shl, Shr, Sub};

use alloc::string::{String, ToString};
use alloc::sync::Arc;
/// We need alloc!
use alloc::vec::Vec;

use crate::atoms::not;

/// This struct is the Err result when parsing.
/// It contains a string representing:
/// The actual input received
/// The expected input
/// And the remaining, unparsed input
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

/// The Output type represents the output of a parser.
/// Ok(T, String) result represents successfully parsed & lexed input.
/// The T type represents the consumed and lexed input,
/// and the String represents the remaining input.
pub type Output<T> = Result<(T, String), Error>;

/// A Parser has a function that consumes input
/// and returns an object of type Output.
#[derive(Clone)]
pub struct Parser<T> {
    parser: Arc<dyn Fn(&str) -> Output<T>>,
}

impl<T> Parser<T>
where
    T: 'static + Clone,
{
    /// Create a new parser from a function that returns an Output.
    /// This is mainly used to define the atomic combinators
    pub fn new(parser: impl Fn(&str) -> Output<T> + 'static) -> Self {
        Self {
            parser: Arc::new(parser),
        }
    }

    /// This parses a string using this combinator, and returns
    /// a result containing either the successfully lexed and parsed
    /// data, or an error containing info about the failure.
    pub fn parse(&self, input: &str) -> Result<T, Error> {
        match self.parse_internal(input) {
            Ok(t) => Ok(t.0),
            Err(e) => Err(e),
        }
    }

    /// This is used by the atomic combinators for things like
    /// control flow and passing the output of one parser into another.
    pub fn parse_internal(&self, input: &str) -> Output<T> {
        (self.parser)(input)
    }

    /// This maps takes a function that takes the output of this Parser,
    /// and converts it to the output of another data type.
    /// This allows us to lex our input as we parse it.
    pub fn map<O>(self, map_fn: fn(T) -> O) -> Parser<O>
    where
        O: 'static + Clone,
    {
        Parser::new(move |s: &str| match self.parse_internal(s) {
            Ok((first_out, input)) => Ok((map_fn(first_out), input)),
            Err(e) => Err(e),
        })
    }

    /// This parser "prefixes" another.
    /// When the returned parser is used, it will require this parser and
    /// the operand parser to succeed, and return the result of the second.
    pub fn prefixes<O>(self, operand: Parser<O>) -> Parser<O>
    where
        O: 'static + Clone,
    {
        Parser::new(move |s: &str| {
            // Get the remaining input from ourselves
            // and discard consumed input
            let (_, remaining) = self.parse_internal(s)?;
            // Get the consumed input and remaining input from operand
            let (consumed, remaining) = operand.parse_internal(&remaining)?;
            // Return result
            Ok((consumed, remaining))
        })
    }

    /// This parser will use the operand as a "suffix".
    /// The parser will only succeed if the "suffix" parser succeeds afterwards,
    /// but the input of the "suffix" parser will be discarded.
    pub fn suffix<O>(self, operand: Parser<O>) -> Parser<T>
    where
        O: 'static + Clone,
    {
        Parser::new(move |s: &str| {
            // Get consumed input and remaining input from ourselves
            let (consumed, remaining) = self.parse_internal(s)?;
            // Consume the input from the remaining,
            // but discard the consumed result.
            let (_, remaining) = operand.parse_internal(&remaining)?;
            // Return result
            Ok((consumed, remaining))
        })
    }

    /// Combine two parsers into one, and combine their consumed
    /// inputs into a tuple.
    /// Parser<A> & Parser<B> -> Parser<A, B>.
    /// The resulting parser will only succeed if BOTH sub-parsers succeed.
    pub fn and<O>(self, operand: Parser<O>) -> Parser<(T, O)>
    where
        O: 'static + Clone,
    {
        Parser::new(move |s: &str| {
            // Get the first consumed and remaining
            let (first_consumed, remaining) = self.parse_internal(s)?;
            // Get the second consumed and remaining
            let (second_consumed, remaining) = operand.parse_internal(&remaining)?;
            // Return a tuple of first and second
            Ok(((first_consumed, second_consumed), remaining))
        })
    }

    /// If this parser does not succeed, try this other parser
    pub fn or(self, operand: Self) -> Self {
        Parser::new(move |s: &str| match self.parse_internal(s) {
            // If we succeed, return OUR result
            Ok(t) => Ok(t),
            // If we don't succeed, return the other parser's result
            // We can safely discard OUR error here because we expect failure.
            Err(_) => operand.parse_internal(s),
        })
    }

    /// Repeat this parser N..M times
    /// This can also be repeated ..N times, N.. times, or even .. times
    pub fn repeat(self, range: impl RangeBounds<usize>) -> Parser<Vec<T>> {
        // Get the upper bound
        let upper_bound: usize = match range.end_bound() {
            Unbounded => &core::usize::MAX,
            Excluded(n) => n,
            Included(n) => n,
        }
        .clone();

        // Get the lower bound
        let lower_bound: usize = match range.start_bound() {
            Unbounded => &0,
            Excluded(n) => n,
            Included(n) => n,
        }
        .clone();

        Parser::new(move |s: &str| {
            // The string containing the remaining input
            let mut remaining_input = s.to_string();
            // This accumulates all the consumed and lexed outputs
            // from all the successfully parsed inputs
            let mut accum = vec![];

            for n in 0..upper_bound {
                match self.parse_internal(&remaining_input) {
                    Ok((consumed, unconsumed)) => {
                        accum.push(consumed);
                        remaining_input = unconsumed;
                    }
                    Err(e) => {
                        // If we did not consume enough data, we failed.
                        // If consumed greater than the lower bound, we succeeded!
                        if n < lower_bound {
                            return Err(e);
                        } else {
                            return Ok((accum, remaining_input));
                        }
                    }
                }
            }
            // Return the vector of consumed inputs
            Ok((accum, remaining_input))
        })
    }
}

/// The | operator can be used as an alternative to the `.or` method
impl<T: 'static + Clone> BitOr for Parser<T> {
    type Output = Parser<T>;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

/// The & operator can be used as an alternative to the `.and` method
impl<A: 'static + Clone, B: 'static + Clone> BitAnd<Parser<B>> for Parser<A> {
    type Output = Parser<(A, B)>;
    fn bitand(self, rhs: Parser<B>) -> Self::Output {
        self.and(rhs)
    }
}

/// The ! operator returns a parser that does not consume input,
/// but succeeds if the input parser does not succeed. This can be
/// used to make assertions for our input.
impl<T: 'static + Clone> Not for Parser<T> {
    type Output = Parser<()>;
    fn not(self) -> Self::Output {
        not(self)
    }
}

/// Discard the consumed data of the RHS
impl<A: 'static + Clone, B: 'static + Clone> Shl<Parser<B>> for Parser<A> {
    type Output = Parser<A>;
    fn shl(self, rhs: Parser<B>) -> Self::Output {
        self.suffix(rhs)
    }
}

/// Discard the consumed data of the LHS
impl<A: 'static + Clone, B: 'static + Clone> Shr<Parser<B>> for Parser<A> {
    type Output = Parser<B>;
    fn shr(self, rhs: Parser<B>) -> Self::Output {
        self.prefixes(rhs)
    }
}

/// A parser can be multiplied by a range as an alternative to `.repeat`.
/// Here's an example: `sym('a') * (..7)`
impl<T: 'static + Clone, R: RangeBounds<usize>> Mul<R> for Parser<T> {
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: R) -> Self::Output {
        self.repeat(rhs)
    }
}

/// The - operator is used as an alternative to the `.map` method.
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
