use std::fmt::Display;
use serde::{Deserialize, Serialize};

use crate::expressions::operator::BinaryOperator;
use crate::parsing::expression_parser::parse_expression;

pub trait OppositeEq {
    fn opposite_eq(&self, other: &Self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Expression {
    Not(Box<Expression>),
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    Atomic(String),
}

impl Expression {
    pub fn is_atomic(&self) -> bool {
        match self {
            Expression::Not(expr) => expr.is_atomic(),
            Expression::Binary { .. } => false,
            Expression::Atomic(_) => true
        }
    }

    pub fn is_not(&self) -> bool {
        matches!(self, Expression::Not(_))
    }

    pub fn exists(&self, atomic_value: &str) -> bool {
        match self {
            Expression::Not(expr) => expr.exists(atomic_value),
            Expression::Binary { left, right, .. } => left.exists(atomic_value) || right.exists(atomic_value),
            Expression::Atomic(value) => value == atomic_value,
        }
    }
}

impl OppositeEq for Expression {
    fn opposite_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Not(_), Expression::Not(_)) => false,
            (Expression::Not(left), right) => left.as_ref() == right,
            (left, Expression::Not(right)) => left == right.as_ref(),
            _ => false,
        }
    }
}

impl<'a> TryFrom<&'a str> for Expression {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        parse_expression(value)
    }
}

impl TryFrom<String> for Expression {
    type Error = nom::Err<nom::error::Error<String>>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
            .map_err(|err| err.map(|err| nom::error::Error::new(err.input.into(), err.code)))
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Not(expr) if expr.is_atomic() => write!(f, "¬{expr}"),
            Expression::Not(expr) => write!(f, "¬({expr})"),
            Expression::Binary { left, operator: BinaryOperator::And, right } => {
                write!(f, "{left} ⋀ {right}")
            }
            // TODO do not use parentheses on root level or if several operators are on the same level
            Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                write!(f, "({left} ⋁ {right})")
            }
            Expression::Binary { left, operator: BinaryOperator::Implication, right } => {
                write!(f, "{left} ➔ {right}")
            }
            Expression::Atomic(value) => write!(f, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{and, atomic, implies, not, or};
    use crate::expressions::expression::Expression;

    #[test]
    fn test_expression_a_and_not_b_display() {
        let expression = and!(
            atomic!("a"),
            not!(atomic!("b"))
        );
        assert_eq!(expression.to_string(), "a ⋀ ¬b");
    }

    #[test]
    #[ignore]
    fn test_expression_a_or_b_and_c_display() {
        // TODO
        let expression = or!(
            atomic!("a"),
            and!(
                atomic!("b"),
                atomic!("c")
            ));
        assert_eq!(expression.to_string(), "a ⋁ b ⋀ c");
    }

    #[test]
    fn test_expression_c_and_a_or_b_display() {
        let expression = and!(
            or!(
                atomic!("a"),
                atomic!("b")
            ),
            atomic!("c")
        );
        assert_eq!(expression.to_string(), "(a ⋁ b) ⋀ c");
    }

    #[test]
    fn test_expression_a_implies_b_display() {
        let expression = implies!(
            atomic!("a"),
            atomic!("b")
        );
        assert_eq!(expression.to_string(), "a ➔ b");
    }

    #[test]
    fn test_expression_not_a_and_b_display() {
        let expression = not!(and!(
            atomic!("a"),
            atomic!("b")
        ));
        assert_eq!(expression.to_string(), "¬(a ⋀ b)");
    }

    #[test]
    fn test_from_str_into_expression_atomic() {
        let expression: Expression = "a".try_into().unwrap();
        assert_eq!(expression, atomic!("a"));
    }

    #[test]
    fn test_from_str_into_expression_not() {
        let expression: Expression = "!a".try_into().unwrap();
        assert_eq!(expression, not!(atomic!("a")));
    }

    #[test]
    fn test_from_str_into_expression_and() {
        let expression: Expression = "a & b".try_into().unwrap();
        assert_eq!(expression, and!(atomic!("a"), atomic!("b")));
    }

    #[test]
    fn test_from_str_into_expression_or() {
        let expression: Expression = "a | b".try_into().unwrap();
        assert_eq!(expression, or!(atomic!("a"), atomic!("b")));
    }

    #[test]
    fn test_from_str_into_expression_implies() {
        let expression: Expression = "a => b".try_into().unwrap();
        assert_eq!(expression, implies!(atomic!("a"), atomic!("b")));
    }

    #[test]
    fn test_from_str_into_expression_complex() {
        let expression: Expression = "a & b | c".try_into().unwrap();
        assert_eq!(expression, or!(and!(atomic!("a"), atomic!("b")), atomic!("c")));
    }

    #[test]
    fn test_from_str_into_expression_complex_parentheses() {
        let expression: Expression = "a & (b | c)".try_into().unwrap();
        assert_eq!(expression, and!(atomic!("a"), or!(atomic!("b"), atomic!("c"))));
    }

    #[test]
    fn test_from_str_into_expression_very_complex_parentheses() {
        let expression: Expression = "(a & b) | c => (d & e)".try_into().unwrap();
        assert_eq!(expression, implies!(or!(and!(atomic!("a"), atomic!("b")), atomic!("c")), and!(atomic!("d"), atomic!("e"))));
    }

    #[test]
    fn test_from_str_into_expression_empty() {
        assert!(Expression::try_from("").is_err());
    }
}
