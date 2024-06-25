use std::rc::Rc;
use crate::expressions::expression::Expression;
use crate::expressions::operator::BinaryOperator;

#[inline]
#[must_use]
pub fn and<L, R>(left: L, right: R) -> Expression
where
    L: Into<Rc<Expression>>,
    R: Into<Rc<Expression>>,
{
    binary(left, BinaryOperator::And, right)
}

#[inline]
#[must_use]
pub fn or<L, R>(left: L, right: R) -> Expression
where
    L: Into<Rc<Expression>>,
    R: Into<Rc<Expression>>,
{
    binary(left, BinaryOperator::Or, right)
}

#[inline]
#[must_use]
pub fn implies<L, R>(left: L, right: R) -> Expression
where
    L: Into<Rc<Expression>>,
    R: Into<Rc<Expression>>,
{
    binary(left, BinaryOperator::Implication, right)
}

#[inline]
#[must_use]
pub fn binary<L, R>(left: L, operator: BinaryOperator, right: R) -> Expression
where
    L: Into<Rc<Expression>>,
    R: Into<Rc<Expression>>,
{
    Expression::Binary { left: left.into(), operator, right: right.into() }
}

#[inline]
#[must_use]
pub fn not<T: Into<Rc<Expression>>>(value: T) -> Expression {
    Expression::Not(value.into())
}

#[inline]
#[must_use]
pub fn atomic<T: Into<String>>(value: T) -> Expression {
    Expression::Atomic(value.into())
}

// TODO eval function using nom parser
