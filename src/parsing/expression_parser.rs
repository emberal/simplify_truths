use lib::nom::combinators::{exhausted, parenthesized, trim};
use lib::traits::IntoResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::{map, opt, peek};
use nom::error::Error;
use nom::sequence::{pair, preceded};
use nom::IResult;

use crate::expressions::expression::Expression;
use crate::expressions::helpers::atomic;

pub fn parse_expression(input: &str) -> Result<Expression, nom::Err<Error<&str>>> {
    exhausted(_parse_expression)(input).into_result()
}

fn _parse_expression(input: &str) -> IResult<&str, Expression> {
    let (remaining, atomic_expression) = left_hand_side(input)?;
    if let (remaining, Some(complex_expression)) =
        opt(expression(atomic_expression.clone()))(remaining)?
    {
        Ok((remaining, complex_expression))
    } else {
        Ok((remaining, atomic_expression))
    }
}

fn left_hand_side(input: &str) -> IResult<&str, Expression> {
    alt((value, not_expression, parenthesized_expression))(input)
}

fn expression<'a>(previous: Expression) -> impl Fn(&'a str) -> IResult<&'a str, Expression> {
    move |input: &'a str| {
        let (remaining, new) = operator_combinators(previous.clone())(input)?;
        if !remaining.is_empty() {
            expression(new.clone())(remaining)
        } else {
            Ok((remaining, new))
        }
    }
}

fn operator_combinators<'a>(
    expression: Expression,
) -> impl FnMut(&'a str) -> IResult<&'a str, Expression> {
    alt((
        implication_expression(expression.clone()),
        or_expression(expression.clone()),
        and_expression(expression.clone()),
        not_expression,
    ))
}

fn parenthesized_expression(input: &str) -> IResult<&str, Expression> {
    parenthesized(|input| {
        let (remaining, atomic) = trim(left_hand_side)(input)?;
        let (remaining, expression) = trim(operator_combinators(atomic))(remaining)?;
        if peek(trim(char(')')))(remaining).is_ok() {
            Ok((remaining, expression))
        } else {
            operator_combinators(expression)(remaining)
        }
    })(input)
}

fn and_expression<'a>(previous: Expression) -> impl FnMut(&'a str) -> IResult<&'a str, Expression> {
    map(preceded(trim(char('&')), left_hand_side), move |right| {
        previous.clone().and(right)
    })
}

fn complete_and(input: &str) -> IResult<&str, Expression> {
    let (remaining, atomic) = value(input)?;
    and_expression(atomic.clone())(remaining)
}

fn or_expression<'a>(previous: Expression) -> impl FnMut(&'a str) -> IResult<&'a str, Expression> {
    map(
        preceded(trim(char('|')), alt((complete_and, left_hand_side))),
        move |right| previous.clone().or(right),
    )
}

fn complete_or(input: &str) -> IResult<&str, Expression> {
    let (remaining, atomic) = value(input)?;
    or_expression(atomic.clone())(remaining)
}

fn implication_expression<'a>(
    previous: Expression,
) -> impl FnMut(&'a str) -> IResult<&'a str, Expression> {
    map(
        preceded(
            trim(tag("=>")),
            alt((complete_and, complete_or, left_hand_side)),
        ),
        move |right| previous.clone().implies(right),
    )
}

fn not_expression(input: &str) -> IResult<&str, Expression> {
    preceded(char('!'), left_hand_side)(input).map(|(remaining, right)| (remaining, right.not()))
}

fn value(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            take_while1(|c: char| c.is_ascii_alphabetic()),
            take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
        ),
        |(first, rest)| {
            let value = format!("{first}{rest}");
            atomic(value)
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::expressions::helpers::*;

    #[test]
    fn test_parse() {
        let input = "a & b => c";
        let result = super::parse_expression(input);
        assert_eq!(
            result,
            Ok(implies(and(atomic("a"), atomic("b")), atomic("c")))
        );
    }

    #[test]
    fn test_parse_complex() {
        let input = "a => b | !(!c | d & e) => b";
        let result = super::parse_expression(input);
        assert_eq!(
            result,
            Ok(implies(
                implies(
                    atomic("a"),
                    or(
                        atomic("b"),
                        not(or(not(atomic("c")), and(atomic("d"), atomic("e")),)),
                    ),
                ),
                atomic("b"),
            ))
        );
    }

    #[test]
    fn test_operator_weight() {
        let input = "A & B | C => D | E & F";
        let result = super::parse_expression(input);
        assert_eq!(
            result,
            Ok(implies(
                or(and(atomic("A"), atomic("B")), atomic("C")),
                or(atomic("D"), and(atomic("E"), atomic("F")))
            ))
        );
    }

    #[test]
    fn test_implies_chain() {
        let input = "a => b => c";
        let result = super::parse_expression(input);
        assert_eq!(
            result,
            Ok(implies(implies(atomic("a"), atomic("b")), atomic("c")))
        );
    }

    #[test]
    fn test_parse_parentheses() {
        let input = "a & (b => c)";
        let result = super::parse_expression(input);
        assert_eq!(
            result,
            Ok(and(atomic("a"), implies(atomic("b"), atomic("c"))))
        );
    }

    #[test]
    fn test_parse_not() {
        let input = "!a";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(not(atomic("a"))));
    }

    #[test]
    fn test_parse_not_parentheses() {
        let input = "!(a & b)";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(not(and(atomic("a"), atomic("b")))));
    }

    #[test]
    fn test_expression_with_not_inside_and() {
        let input = "a & !b";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(and(atomic("a"), not(atomic("b")))));
    }

    #[test]
    fn test_expression_with_not_inside_or() {
        let input = "a | !b";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(or(atomic("a"), not(atomic("b")))));
    }

    #[test]
    fn test_expression_with_not_inside_implies() {
        let input = "a => !b";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(implies(atomic("a"), not(atomic("b")))));
    }

    #[test]
    fn test_expression_with_not_inside_parentheses() {
        let input = "a & !(b | c)";
        let result = super::parse_expression(input);
        assert_eq!(
            result,
            Ok(and(atomic("a"), not(or(atomic("b"), atomic("c")))))
        );
    }

    #[test]
    fn test_even_not() {
        let input = "!!!!a";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(not(not(not(not(atomic("a")))))));
    }

    #[test]
    fn test_odd_not() {
        let input = "!!!!!a";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(not(not(not(not(not(atomic("a"))))))));
    }

    #[test]
    fn test_atomic() {
        let input = "a";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(atomic("a")));
    }

    #[test]
    fn test_atomic_with_underscore() {
        let input = "a_b";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(atomic("a_b")));
    }

    #[test]
    fn test_atomic_with_digits() {
        let input = "a1";
        let result = super::parse_expression(input);
        assert_eq!(result, Ok(atomic("a1")));
    }

    #[test]
    fn test_empty() {
        let input = "";
        let result = super::parse_expression(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_or_chain() {
        let input = "a | b | c | d | e | f | g";
        let result = super::parse_expression(input);
        assert_eq!(
            result,
            Ok(or(
                or(
                    or(
                        or(or(or(atomic("a"), atomic("b")), atomic("c")), atomic("d")),
                        atomic("e")
                    ),
                    atomic("f")
                ),
                atomic("g")
            ))
        );
    }

    #[test]
    fn test_expression() {
        let input = "a";
        let result = super::_parse_expression(input);
        assert_eq!(result, Ok(("", atomic("a"))));
    }

    #[test]
    fn test_expression_and() {
        let expression = atomic("a");
        let input = " & b";
        let result = super::expression(expression)(input);
        assert_eq!(result, Ok(("", and(atomic("a"), atomic("b")))));
    }

    #[test]
    fn test_expression_and_or() {
        let expression = atomic("a");
        let input = " & b | c";
        let result = super::expression(expression)(input);
        assert_eq!(
            result,
            Ok(("", or(and(atomic("a"), atomic("b")), atomic("c"))))
        );
    }

    #[test]
    fn test_expression_and_or_implies() {
        let expression = atomic("a");
        let input = " & b | c => d";
        let result = super::expression(expression)(input);
        assert_eq!(
            result,
            Ok((
                "",
                implies(or(and(atomic("a"), atomic("b")), atomic("c")), atomic("d"))
            ))
        );
    }

    #[test]
    fn test_expression_parentheses_or() {
        let expression = atomic("a");
        let input = " & (b | c) => d";
        let result = super::expression(expression)(input);
        assert_eq!(
            result,
            Ok((
                "",
                implies(and(atomic("a"), or(atomic("b"), atomic("c"))), atomic("d"))
            ))
        );
    }

    #[test]
    fn test_expression_parentheses_and() {
        let input = "(a & b) | (c & d)";
        let result = super::_parse_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                or(and(atomic("a"), atomic("b")), and(atomic("c"), atomic("d")))
            ))
        );
    }

    #[test]
    fn test_expression_parentheses_implies() {
        let expression = atomic("a");
        let input = " & b | (c => d)";
        let result = super::expression(expression)(input);
        assert_eq!(
            result,
            Ok((
                "",
                or(
                    and(atomic("a"), atomic("b")),
                    implies(atomic("c"), atomic("d"))
                )
            ))
        );
    }

    #[test]
    fn test_expression_nested_parentheses() {
        let expression = atomic("a");
        let input = " & (b | (c => d))";
        let result = super::expression(expression)(input);
        assert_eq!(
            result,
            Ok((
                "",
                and(
                    atomic("a"),
                    or(atomic("b"), implies(atomic("c"), atomic("d")))
                )
            ))
        );
    }

    #[test]
    fn test_parse_or() {
        let expression = atomic("a");
        let input = " | b";
        let result = super::or_expression(expression)(input);
        assert_eq!(result, Ok(("", or(atomic("a"), atomic("b")))));
    }

    #[test]
    fn test_parse_or_parentheses() {
        let input = "(a | b)";
        let result = super::_parse_expression(input);
        assert_eq!(result, Ok(("", or(atomic("a"), atomic("b")))));
    }

    #[test]
    fn test_parenthesized_expression_3_atomics() {
        let input = "(A | B | C)";
        let result = super::parenthesized_expression(input);
        assert_eq!(
            result,
            Ok(("", or(or(atomic("A"), atomic("B")), atomic("C"))))
        );
    }
}
