use std::collections::HashSet;
use std::fmt::Display;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::expressions::operator::BinaryOperator;
use crate::parsing::expression_parser::parse_expression;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Expression {
    Not(Rc<Expression>),
    Binary { left: Rc<Expression>, operator: BinaryOperator, right: Rc<Expression> },
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

    pub fn get_atomic_values(&self) -> HashSet<String> {
        match self {
            Expression::Not(expr) => expr.get_atomic_values(),
            Expression::Binary { left, right, .. } => {
                let mut values = left.get_atomic_values();
                values.extend(right.get_atomic_values());
                values
            }
            Expression::Atomic(value) => HashSet::from([value.clone()])
        }
    }

    pub fn eq(&self, other: &Self, ignore_case: bool) -> bool {
        match (self, other) {
            (Expression::Not(left), Expression::Not(right)) => Expression::eq(left, right, ignore_case),
            (Expression::Binary { left: left_left, operator: left_operator, right: left_right },
                Expression::Binary { left: right_left, operator: right_operator, right: right_right }) => {
                Expression::eq(left_left, right_left, ignore_case)
                    && left_operator == right_operator
                    && Expression::eq(left_right, right_right, ignore_case)
            }
            (Expression::Atomic(left), Expression::Atomic(right)) => {
                if ignore_case {
                    left.eq_ignore_ascii_case(right)
                } else {
                    left == right
                }
            }
            _ => false
        }
    }

    pub fn opposite_eq(&self, other: &Self, ignore_case: bool) -> bool {
        match (self, other) {
            (Expression::Not(_), Expression::Not(_)) => false,
            (Expression::Not(left), right) => left.as_ref().eq(right, ignore_case),
            (left, Expression::Not(right)) => left.eq(right.as_ref(), ignore_case),
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
        return write!(f, "{}", fmt_helper(self, None));

        fn fmt_helper(expression: &Expression, parent: Option<&Expression>) -> String {
            match expression {
                Expression::Not(expr) if expr.is_atomic() => format!("¬{}", fmt_helper(expr, Some(expression))),
                Expression::Not(expr) => format!("¬({})", fmt_helper(expr, Some(expression))),
                Expression::Binary { left, operator: BinaryOperator::And, right } => {
                    format!("{} ⋀ {}", fmt_helper(left, Some(expression)), fmt_helper(right, Some(expression)))
                }
                Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                    if parent.is_none() || matches!(parent, Some(Expression::Binary { operator: BinaryOperator::Or | BinaryOperator::Implication, .. })) {
                        format!("{} ⋁ {}", fmt_helper(left, Some(expression)), fmt_helper(right, Some(expression)))
                    } else {
                        format!("({} ⋁ {})", fmt_helper(left, Some(expression)), fmt_helper(right, Some(expression)))
                    }
                }
                Expression::Binary { left, operator: BinaryOperator::Implication, right } => {
                    format!("{} ➔ {}", fmt_helper(left, Some(expression)), fmt_helper(right, Some(expression)))
                }
                Expression::Atomic(value) => value.clone(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::expression::Expression;
    use crate::expressions::helpers::{and, atomic, implies, not, or};

    #[test]
    fn test_eq_ignore_case_atomics() {
        let expression_lower = atomic("a");
        let expression_upper = atomic("A");
        assert!(expression_lower.eq(&expression_upper, true));
    }

    #[test]
    fn test_eq_ignore_case_not() {
        let expression_lower = not(atomic("a"));
        let expression_upper = not(atomic("A"));
        assert!(expression_lower.eq(&expression_upper, true));
    }

    #[test]
    fn test_eq_ignore_case_and() {
        let expression_lower = and(atomic("a"), atomic("b"));
        let expression_upper = and(atomic("A"), atomic("B"));
        assert!(expression_lower.eq(&expression_upper, true));
    }

    #[test]
    fn test_eq_ignore_case_equal() {
        let expression_lower = or(atomic("a"), atomic("b"));
        let expression_upper = or(atomic("a"), atomic("b"));
        assert!(expression_lower.eq(&expression_upper, true));
    }

    #[test]
    fn test_eq_ignore_case_unequal() {
        let expression_lower = or(atomic("a"), atomic("b"));
        let expression_upper = or(atomic("a"), atomic("c"));
        assert!(!expression_lower.eq(&expression_upper, true));
    }

    #[test]
    fn test_eq_dont_ignore_case() {
        let expression_lower = or(atomic("a"), atomic("b"));
        let expression_upper = or(atomic("a"), atomic("B"));
        assert!(!expression_lower.eq(&expression_upper, false));
    }

    #[test]
    fn test_expression_a_and_not_b_display() {
        let expression = and(
            atomic("a"),
            not(atomic("b")),
        );
        assert_eq!(expression.to_string(), "a ⋀ ¬b");
    }

    #[test]
    fn test_expression_a_or_b_and_c_display() {
        let expression = or(
            atomic("a"),
            and(
                atomic("b"),
                atomic("c"),
            ));
        assert_eq!(expression.to_string(), "a ⋁ b ⋀ c");
    }

    #[test]
    fn test_expression_a_or_b() {
        let expression = or(
            atomic("a"),
            atomic("b"),
        );
        assert_eq!(expression.to_string(), "a ⋁ b");
    }

    #[test]
    fn test_expression_double_or() {
        let expression = or(
            atomic("a"),
            or(
                atomic("b"),
                atomic("c"),
            ),
        );
        assert_eq!(expression.to_string(), "a ⋁ b ⋁ c");
    }

    #[test]
    fn test_expression_triple_or() {
        let expression = or(
            atomic("a"),
            or(
                atomic("b"),
                or(
                    atomic("c"),
                    atomic("d"),
                ),
            ),
        );
        assert_eq!(expression.to_string(), "a ⋁ b ⋁ c ⋁ d");
    }

    #[test]
    fn test_expression_nested_parenthesized_or() {
        let expression = or(
            atomic("a"),
            and(
                atomic("b"),
                or(
                    atomic("b"),
                    atomic("c"),
                ),
            ),
        );
        assert_eq!(expression.to_string(), "a ⋁ b ⋀ (b ⋁ c)");
    }

    #[test]
    fn test_expression_c_and_a_or_b_display() {
        let expression = and(
            or(
                atomic("a"),
                atomic("b"),
            ),
            atomic("c"),
        );
        assert_eq!(expression.to_string(), "(a ⋁ b) ⋀ c");
    }

    #[test]
    fn test_expression_a_implies_b_display() {
        let expression = implies(
            atomic("a"),
            atomic("b"),
        );
        assert_eq!(expression.to_string(), "a ➔ b");
    }

    #[test]
    fn test_expression_not_a_and_b_display() {
        let expression = not(and(
            atomic("a"),
            atomic("b"),
        ));
        assert_eq!(expression.to_string(), "¬(a ⋀ b)");
    }

    #[test]
    fn test_from_str_into_expression_atomic() {
        let expression: Expression = "a".try_into().unwrap();
        assert_eq!(expression, atomic("a"));
    }

    #[test]
    fn test_from_str_into_expression_not() {
        let expression: Expression = "!a".try_into().unwrap();
        assert_eq!(expression, not(atomic("a")));
    }

    #[test]
    fn test_from_str_into_expression_and() {
        let expression: Expression = "a & b".try_into().unwrap();
        assert_eq!(expression, and(atomic("a"), atomic("b")));
    }

    #[test]
    fn test_from_str_into_expression_or() {
        let expression: Expression = "a | b".try_into().unwrap();
        assert_eq!(expression, or(atomic("a"), atomic("b")));
    }

    #[test]
    fn test_from_str_into_expression_implies() {
        let expression: Expression = "a => b".try_into().unwrap();
        assert_eq!(expression, implies(atomic("a"), atomic("b")));
    }

    #[test]
    fn test_from_str_into_expression_complex() {
        let expression: Expression = "a & b | c".try_into().unwrap();
        assert_eq!(expression, or(and(atomic("a"), atomic("b")), atomic("c")));
    }

    #[test]
    fn test_from_str_into_expression_complex_parentheses() {
        let expression: Expression = "a & (b | c)".try_into().unwrap();
        assert_eq!(expression, and(atomic("a"), or(atomic("b"), atomic("c"))));
    }

    #[test]
    fn test_from_str_into_expression_very_complex_parentheses() {
        let expression: Expression = "(a & b) | c => (d & e)".try_into().unwrap();
        assert_eq!(expression, implies(or(and(atomic("a"), atomic("b")), atomic("c")), and(atomic("d"), atomic("e"))));
    }

    #[test]
    fn test_from_str_into_expression_empty() {
        assert!(Expression::try_from("").is_err());
    }
}
