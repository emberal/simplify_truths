use std::ops::Deref;
use std::rc::Rc;
use crate::expressions::expression::Expression;

pub struct ExpressionIterator {
    stack: Vec<Rc<Expression>>,
}

impl ExpressionIterator {
    pub fn new(expression: Expression) -> Self {
        let stack = vec![expression.into()];
        Self { stack }
    }
}

impl Iterator for ExpressionIterator {
    type Item = Rc<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        let expression = self.stack.pop()?;
        match expression.deref() {
            Expression::Atomic(_) => Some(expression),
            Expression::Not(inner) => {
                self.stack.push(inner.clone());
                Some(expression)
            }
            Expression::Binary { left, right, .. } => {
                self.stack.push(right.clone());
                self.stack.push(left.clone());
                Some(expression)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::helpers::{and, atomic, not};

    #[test]
    fn test_expression_iterator() {
        let expression = not(and(atomic("A"), atomic("B")));
        let mut iterator = expression.iter();
        assert_eq!(iterator.next().unwrap(), expression.into());
        assert_eq!(iterator.next().unwrap(), and(atomic("A"), atomic("B")).into());
        assert_eq!(iterator.next().unwrap(), atomic("A").into());
        assert_eq!(iterator.next().unwrap(), atomic("B").into());
        assert_eq!(iterator.next(), None);
    }
}
