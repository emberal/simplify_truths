use std::ops::RangeFrom;

use nom::{InputIter, InputLength, InputTake, IResult, Slice};
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::{char, multispace0};
use nom::combinator::eof;
use nom::error::{Error, ParseError};
use nom::sequence::{delimited, terminated};

/// Trim leading and trailing whitespace from the input Parser
/// - Parameters
///    - `inner`: The parser to trim
/// - Returns: A parser that trims leading and trailing whitespace from the input and then runs the value from the inner parser
pub fn trim<'a, Parser, R>(inner: Parser) -> impl FnMut(&'a str) -> IResult<&'a str, R>
    where
        Parser: Fn(&'a str) -> IResult<&'a str, R> {
    delimited(
        multispace0,
        inner,
        multispace0,
    )
}

/// Parse a parenthesized expression. This parser will parse an expression that is surrounded by parentheses
/// and will trim the whitespace surrounding the expression.
/// - Parameters
///     - `inner`: The parser to run inside the parentheses
/// - Returns: A parser that parses a parenthesized expression
pub fn parenthesized<'a, Parser, R>(inner: Parser) -> impl FnMut(&'a str) -> IResult<&'a str, R>
    where
        Parser: Fn(&'a str) -> IResult<&'a str, R> {
    delimited(
        char('('),
        trim(inner),
        char(')'),
    )
}

/// Take where the predicate is true and the length is exactly `n`
/// - Parameters
///   - `n`: The length of the string to take
///   - `predicate`: The predicate to call to validate the input
/// - Returns: A parser that takes `n` characters from the input
pub fn take_where<F, Input, Error: ParseError<Input>>(n: usize, predicate: F) -> impl Fn(Input) -> IResult<Input, Input, Error>
    where Input: InputTake + InputIter + InputLength + Slice<RangeFrom<usize>>, F: Fn(<Input as InputIter>::Item) -> bool, {
    move |input: Input| {
        take_while_m_n(n, n, |it| predicate(it))(input)
    }
}

pub fn exhausted<'a, Parser, R>(inner: Parser) -> impl FnMut(&'a str) -> IResult<&'a str, R>
    where
        Parser: Fn(&'a str) -> IResult<&'a str, R> {
    terminated(inner, eof)
}

pub trait IntoResult<T> {
    type Error;
    fn into_result(self) -> Result<T, Self::Error>;
}

impl<T, R> IntoResult<T> for IResult<R, T> {
    type Error = nom::Err<Error<R>>;
    fn into_result(self) -> Result<T, Self::Error> {
        self.map(|(_remaining, value)| value)
    }
}

#[cfg(test)]
mod tests {
    use nom::bytes::streaming::take_while;
    use crate::parsing::utils::{exhausted, parenthesized, take_where};

    #[test]
    fn test_parenthesized() {
        let input = "(test)";
        let (remaining, result) = parenthesized(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_parenthesized_parse_until_end() {
        let input = "(test)";
        assert!(parenthesized(take_while(|_| true))(input).is_err());
    }

    #[test]
    fn test_exhausted() {
        let input = "test";
        let (remaining, result) = exhausted(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_exhausted_not_exhausted() {
        let input = "test ";
        assert!(exhausted(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).is_err());
    }
}
