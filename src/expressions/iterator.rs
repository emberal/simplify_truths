use crate::expressions::expression::Expression;

pub struct ExpressionIterator {
    stack: Vec<Expression>,
}

impl ExpressionIterator {
    pub fn new(expression: Expression) -> Self {
        let stack = vec![expression];
        Self { stack }
    }
}

impl Iterator for ExpressionIterator {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(expression) = self.stack.pop() {
            match &expression {
                Expression::Atomic(_) => Some(expression),
                Expression::Not(inner) => {
                    self.stack.push(*inner.clone());
                    Some(expression)
                }
                Expression::Binary { left, right, .. } => {
                    self.stack.push(*right.clone());
                    self.stack.push(*left.clone());
                    Some(expression)
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_expression_iterator() {
        let expression = not!(and!(atomic!("A"), atomic!("B")));
        let mut iterator = expression.iter();
        assert_eq!(iterator.next().unwrap(), expression);
        assert_eq!(iterator.next().unwrap(), and!(atomic!("A"), atomic!("B")));
        assert_eq!(iterator.next().unwrap(), atomic!("A"));
        assert_eq!(iterator.next().unwrap(), atomic!("B"));
        assert_eq!(iterator.next(), None);
    }
}
