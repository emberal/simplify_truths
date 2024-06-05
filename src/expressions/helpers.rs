#[macro_export]
macro_rules! and {
    ($left:expr, $right:expr) => {
        $crate::binary!($left, $crate::expressions::operator::BinaryOperator::And, $right)
    };
}

#[macro_export]
macro_rules! or {
    ($left:expr, $right:expr) => {
        $crate::binary!($left, $crate::expressions::operator::BinaryOperator::Or, $right)
    };
}

#[macro_export]
macro_rules! implies {
    ($left:expr, $right:expr) => {
        $crate::binary!($left, $crate::expressions::operator::BinaryOperator::Implication, $right)
    };
}

#[macro_export]
macro_rules! binary {
    ($left:expr, $op:expr, $right:expr) => {
        $crate::expressions::expression::Expression::Binary(Box::new($left), $op, Box::new($right))
    };
}

#[macro_export]
macro_rules! not {
    ($value:expr) => {
        $crate::expressions::expression::Expression::Not(Box::new($value))
    };
}

#[macro_export]
macro_rules! atomic {
    ($value:expr) => {
        $crate::expressions::expression::Expression::Atomic($value.to_string())
    };
}

// TODO
#[macro_export]
macro_rules! eval {
    ($a:literal && $b:literal) => {
        $crate::and!($crate::eval!($a), $crate::eval!($b))
    };
    ($a:literal || $b:literal) => {
        $crate::or!($crate::eval!($a), $crate::eval!($b))
    };
    ($a:literal => $b:literal) => {
        $crate::implies!($crate::eval!($a), $crate::eval!($b))
    };
    (!$a:expr) => {
        $crate::not!($crate::eval!($a))
    };
    ($value:expr) => {
        $crate::atomic!($value)
    };
}

#[cfg(test)]
mod tests {
    use crate::eval;
    use crate::expressions::expression::Expression::{Atomic, Binary, Not};
    use crate::expressions::operator::BinaryOperator::{And, Implication, Or};

    #[test]
    fn eval_atomic() {
        assert_eq!(eval!("a"), Atomic("a".to_string()));
    }

    #[test]
    fn eval_not() {
        assert_eq!(eval!(!"a"), Not(Box::new(Atomic("a".to_string()))));
    }

    #[test]
    fn eval_and() {
        assert_eq!(eval!("a" && "b"), Binary(Box::new(Atomic("a".to_string())), And, Box::new(Atomic("b".to_string()))));
    }

    #[test]
    fn eval_or() {
        assert_eq!(eval!("a" || "b"), Binary(Box::new(Atomic("a".to_string())), Or, Box::new(Atomic("b".to_string()))));
    }

    #[test]
    fn eval_implies() {
        assert_eq!(eval!("a" => "b"), Binary(Box::new(Atomic("a".to_string())), Implication, Box::new(Atomic("b".to_string()))));
    }
}
