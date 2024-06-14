use std::rc::Rc;
use crate::expressions::expression::Expression;
use crate::expressions::operator::BinaryOperator;

#[inline]
pub fn and<L, R>(left: L, right: R) -> Expression
where
    L: Into<Rc<Expression>>,
    R: Into<Rc<Expression>>,
{
    binary(left, BinaryOperator::And, right)
}

#[inline]
pub fn or<L, R>(left: L, right: R) -> Expression
where
    L: Into<Rc<Expression>>,
    R: Into<Rc<Expression>>,
{
    binary(left, BinaryOperator::Or, right)
}

#[inline]
pub fn implies<L, R>(left: L, right: R) -> Expression
where
    L: Into<Rc<Expression>>,
    R: Into<Rc<Expression>>,
{
    binary(left, BinaryOperator::Implication, right)
}

#[inline]
pub fn binary<L, R>(left: L, operator: BinaryOperator, right: R) -> Expression
where
    L: Into<Rc<Expression>>,
    R: Into<Rc<Expression>>,
{
    Expression::Binary { left: left.into(), operator, right: right.into() }
}

#[inline]
pub fn not<T: Into<Rc<Expression>>>(value: T) -> Expression {
    Expression::Not(value.into())
}

#[inline]
pub fn atomic<T: Into<String>>(value: T) -> Expression {
    Expression::Atomic(value.into())
}

// TODO eval function using nom parser
