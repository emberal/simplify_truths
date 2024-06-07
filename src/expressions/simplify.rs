use crate::expressions::expression::{Expression, OppositeEq};
use crate::expressions::operator::BinaryOperator;

pub trait Simplify {
    fn simplify(&self) -> Self;
    fn elimination_of_implication(&self) -> Self;
    fn double_negation_elimination(&self) -> Self;
    fn de_morgans_laws(&self) -> Self;
    fn absorption_law(&self) -> Self;
    fn associative_law(&self) -> Self;
    fn distribution_law(&self) -> Self;
    fn commutative_law(&self) -> Self;
}

impl Simplify for Expression {
    // TODO test and define order of operations
    fn simplify(&self) -> Self {
        self.elimination_of_implication()
            .de_morgans_laws()
            .absorption_law()
            // .associative_law()
            .distribution_law()
            .double_negation_elimination()
        // .commutative_law()
    }
    /// Eliminate the implication operator from the expression.
    /// This is done by replacing `a ➔ b` with `¬a ⋁ b`.
    fn elimination_of_implication(&self) -> Self {
        match self {
            Expression::Not(expr) => not!(expr.elimination_of_implication()),
            Expression::Binary { left, operator: BinaryOperator::Implication, right } => {
                let left = left.elimination_of_implication();
                let right = right.elimination_of_implication();
                or!(not!(left), right)
            }
            Expression::Binary { left, operator, right } => {
                let left = left.elimination_of_implication();
                let right = right.elimination_of_implication();
                binary!(left, *operator, right)
            }
            atomic @ Expression::Atomic(_) => atomic.clone(),
        }
    }

    /// Eliminate double negations from the expression.
    /// This is done by replacing `¬¬a` with `a`.
    /// This function is recursive and will continue to eliminate double negations until none are left.
    fn double_negation_elimination(&self) -> Self {
        match self {
            Expression::Not(expr) => {
                if let Expression::Not(inner) = *expr.clone() {
                    inner.double_negation_elimination()
                } else {
                    not!(expr.double_negation_elimination())
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.double_negation_elimination();
                let right = right.double_negation_elimination();
                binary!(left, *operator, right)
            }
            atomic @ Expression::Atomic(_) => atomic.clone(),
        }
    }

    fn de_morgans_laws(&self) -> Self {
        match self {
            Expression::Not(expr) => {
                match *expr.clone() {
                    Expression::Binary { left, operator: BinaryOperator::And, right } => {
                        // TODO unnecessary cloning calls to de_morgans_laws?
                        let left = not!(left.de_morgans_laws());
                        let right = not!(right.de_morgans_laws());
                        or!(left, right).de_morgans_laws()
                    }
                    Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                        let left = not!(left.de_morgans_laws());
                        let right = not!(right.de_morgans_laws());
                        and!(left, right).de_morgans_laws()
                    }
                    _ => not!(expr.de_morgans_laws()),
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.de_morgans_laws();
                let right = right.de_morgans_laws();
                binary!(left, *operator, right)
            }
            atomic @ Expression::Atomic(_) => atomic.clone(),
        }
    }

    // TODO deduplicate code
    fn absorption_law(&self) -> Self {
        match self {
            Expression::Binary { left, operator: BinaryOperator::And, right } => {
                let (left_ref, right_ref) = (left.as_ref(), right.as_ref());
                match (left_ref, right_ref) {
                    (_, Expression::Binary { left: right_left, operator: BinaryOperator::Or, right: right_right }) => {
                        if left_ref == right_left.as_ref() || left_ref == right_right.as_ref() {
                            return left.absorption_law();
                        } else if right_left.is_atomic() && right_right.is_atomic() && left.opposite_eq(right_left) {
                            if left.opposite_eq(right_left) {
                                return and!(left.absorption_law(), right_left.absorption_law());
                            } else if left.opposite_eq(right_right) {
                                return and!(left.absorption_law(), right_right.absorption_law());
                            }
                        }
                        and!(left.absorption_law(), right.absorption_law())
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::Or, right: left_right }, _) => {
                        if right_ref == left_left.as_ref() || right_ref == left_right.as_ref() {
                            return right.absorption_law();
                        } else if left_left.is_atomic() && left_right.is_atomic() && right.opposite_eq(left_left) {
                            if right.opposite_eq(left_left) {
                                return and!(left_right.absorption_law(), right.absorption_law());
                            } else if right.opposite_eq(left_right) {
                                return and!(left_left.absorption_law(), right.absorption_law());
                            }
                        }
                        and!(left.absorption_law(), right.absorption_law())
                    }
                    (left, right) => and!(left.absorption_law(), right.absorption_law())
                }
            }
            Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                let (left_ref, right_ref) = (left.as_ref(), right.as_ref());
                match (left_ref, right_ref) {
                    (_, Expression::Binary { left: right_left, operator: BinaryOperator::And, right: right_right }) => {
                        if left_ref == right_left.as_ref() || left_ref == right_right.as_ref() {
                            return left.absorption_law();
                        } else if right_left.is_atomic() && right_right.is_atomic() && left.opposite_eq(right_left) {
                            if left.opposite_eq(right_left) {
                                return or!(left.absorption_law(), right_left.absorption_law());
                            } else if left.opposite_eq(right_right) {
                                return or!(left.absorption_law(), right_right.absorption_law());
                            }
                        }
                        or!(left.absorption_law(), right.absorption_law())
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::And, right: left_right }, _) => {
                        if right_ref == left_left.as_ref() || right_ref == left_right.as_ref() {
                            return right.absorption_law();
                        } else if left_left.is_atomic() && left_right.is_atomic() && right.opposite_eq(left_left) {
                            if right.opposite_eq(left_left) {
                                return or!(left_right.absorption_law(), right.absorption_law());
                            } else if right.opposite_eq(left_right) {
                                return or!(left_left.absorption_law(), right.absorption_law());
                            }
                        }
                        or!(left.absorption_law(), right.absorption_law())
                    }
                    (left, right) => or!(left.absorption_law(), right.absorption_law())
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.absorption_law();
                let right = right.absorption_law();
                binary!(left, *operator, right)
            }
            Expression::Not(expr) => not!(expr.absorption_law()),
            atomic => atomic.clone(),
        }
    }

    fn associative_law(&self) -> Self {
        todo!("? | Associative law: (a ⋀ b) ⋀ c == a ⋀ (b ⋀ c) and (a ⋁ b) ⋁ c == a ⋁ (b ⋁ c)")
    }

    // TODO deduplicate code
    fn distribution_law(&self) -> Self {
        match self {
            Expression::Binary { left, operator: BinaryOperator::And, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (Expression::Atomic(_), Expression::Binary { left: right_left, operator: BinaryOperator::Or, right: right_right }) => {
                        let right_left = right_left.distribution_law();
                        let right_right = right_right.distribution_law();
                        or!(and!(*left.clone(), right_left), and!(*left.clone(), right_right))
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::Or, right: left_right }, Expression::Atomic(_)) => {
                        let left_left = left_left.distribution_law();
                        let left_right = left_right.distribution_law();
                        or!(and!(left_left, *right.clone()), and!(left_right, *right.clone()))
                    }
                    (left, right) => and!(left.distribution_law(), right.distribution_law())
                }
            }
            Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (Expression::Atomic(_), Expression::Binary { left: right_left, operator: BinaryOperator::And, right: right_right }) => {
                        let right_left = right_left.distribution_law();
                        let right_right = right_right.distribution_law();
                        and!(or!(*left.clone(), right_left), or!(*left.clone(), right_right))
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::And, right: left_right }, Expression::Atomic(_)) => {
                        let left_left = left_left.distribution_law();
                        let left_right = left_right.distribution_law();
                        and!(or!(left_left, *right.clone()), or!(left_right, *right.clone()))
                    }
                    (left, right) => or!(left.distribution_law(), right.distribution_law())
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.distribution_law();
                let right = right.distribution_law();
                binary!(left, *operator, right)
            }
            Expression::Not(expr) => not!(expr.distribution_law()),
            atomic => atomic.clone(),
        }
    }

    fn commutative_law(&self) -> Self {
        todo!("? | Order of operands does not matter in AND and OR operations.")
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::simplify::Simplify;

    #[test]
    fn test_simplify() {
        let expression = eval!("a" => "b").simplify();
        assert_eq!(expression, or!(not!(atomic!("a")), atomic!("b")));
    }

    #[test]
    fn test_implication_and_de_morgans() {
        let expression = implies!(and!(not!(atomic!("a")), atomic!("b")), atomic!("c")).simplify();
        assert_eq!(expression, or!(or!(atomic!("a"), not!(atomic!("b"))), atomic!("c")));
    }

    #[test]
    fn test_elimination_of_implication() {
        let expression = eval!("a" => "b").elimination_of_implication();
        assert_eq!(expression, or!(not!(atomic!("a")), atomic!("b")));
    }

    #[test]
    fn test_elimination_of_implication_nested() {
        let expression = implies!(atomic!("a"), implies!(atomic!("b"), atomic!("c"))).elimination_of_implication();
        assert_eq!(expression, or!(not!(atomic!("a")), or!(not!(atomic!("b")), atomic!("c"))));
    }

    #[test]
    fn test_elimination_of_implication_none() {
        let expression = eval!("a" && "b").elimination_of_implication();
        assert_eq!(expression, eval!("a" && "b"));
    }

    #[test]
    fn test_elimination_of_implication_nested_none() {
        let expression = or!(atomic!("a"), and!(atomic!("b"), atomic!("c"))).elimination_of_implication();
        assert_eq!(expression, or!(atomic!("a"), and!(atomic!("b"), atomic!("c"))));
    }

    #[test]
    fn test_double_negation_elimination() {
        let expression = not!(not!(atomic!("a"))).double_negation_elimination();
        assert_eq!(expression, atomic!("a"));
    }

    #[test]
    fn test_triple_negation_elimination() {
        let expression = not!(not!(not!(atomic!("a")))).double_negation_elimination();
        assert_eq!(expression, not!(atomic!("a")));
    }

    #[test]
    fn test_five_negation_elimination() {
        let expression = not!(not!(not!(not!(not!(atomic!("a")))))).double_negation_elimination();
        assert_eq!(expression, not!(atomic!("a")));
    }

    #[test]
    fn test_no_negation_elimination() {
        let expression = atomic!("a").double_negation_elimination();
        assert_eq!(expression, atomic!("a"));
    }

    #[test]
    fn test_double_negation_nested_elimination() {
        let expression = and!(or!(not!(eval!(!"a")), eval!("b")), not!(eval!(!"c"))).double_negation_elimination();
        assert_eq!(expression, and!(or!(atomic!("a"), atomic!("b")), atomic!("c")));
    }

    #[test]
    fn test_de_morgans_laws_and() {
        let expression = not!(eval!("a" && "b")).de_morgans_laws();
        assert_eq!(expression, or!(not!(atomic!("a")), not!(atomic!("b"))));
    }

    #[test]
    fn test_de_morgans_laws_or() {
        let expression = not!(eval!("a" || "b")).de_morgans_laws();
        assert_eq!(expression, and!(not!(atomic!("a")), not!(atomic!("b"))));
    }

    #[test]
    fn test_de_morgans_laws_nested_or() {
        let expression = not!(or!(eval!("a" && "b"), atomic!("c"))).de_morgans_laws(); // ¬(a ⋀ b ⋁ c)
        assert_eq!(expression, and!(or!(eval!(!"a"), eval!(!"b")), eval!(!"c"))); // ¬(a ⋀ b) ⋀ ¬c == (¬a ⋁ ¬b) ⋀ ¬c
    }

    #[test]
    fn test_de_morgans_laws_nested_and() {
        let expression = not!(and!(eval!("a" || "b"), atomic!("c"))).de_morgans_laws(); // ¬(a ⋁ b ⋀ c)
        assert_eq!(expression, or!(and!(eval!(!"a"), eval!(!"b")), eval!(!"c"))); // ¬(a ⋁ b) ⋀ ¬c == (¬a ⋀ ¬b) ⋁ ¬c
    }

    #[test]
    fn test_de_morgans_laws_nested_and_or() {
        let expression = not!(and!(eval!("a" || "b"), or!(atomic!("c"), atomic!("d")))).de_morgans_laws(); // ¬(a ⋁ b ⋀ c ⋁ d)
        assert_eq!(expression, or!(and!(eval!(!"a"), eval!(!"b")), and!(eval!(!"c"), eval!(!"d")))); // ¬(a ⋁ b) ⋀ ¬(c ⋁ d) == (¬a ⋀ ¬b) ⋁ (¬c ⋀ ¬d)
    }

    #[test]
    fn test_absorption_law_and() {
        let expression = and!(atomic!("a"), eval!("a" || "b")).absorption_law();
        assert_eq!(expression, atomic!("a"));
    }

    #[test]
    fn test_absorption_law_or() {
        let expression = or!(atomic!("a"), eval!("a" && "b")).absorption_law();
        assert_eq!(expression, atomic!("a"));
    }

    #[test]
    fn test_absorption_law_nested_and() {
        let expression = and!(atomic!("a"), or!(atomic!("a"), atomic!("b"))).absorption_law();
        assert_eq!(expression, atomic!("a"));
    }

    // !A & B | A <=> B | A
    #[test]
    fn test_absorption_law_not() {
        let expression = or!(and!(not!(atomic!("a")), atomic!("b")), atomic!("a")).absorption_law();
        assert_eq!(expression, or!(atomic!("b"), atomic!("a")));
    }

    // A & B | !A <=> B | !A
    #[test]
    fn test_absorption_law_not_reversed() {
        let expression = or!(and!(atomic!("a"), atomic!("b")), not!(atomic!("a"))).absorption_law();
        assert_eq!(expression, or!(atomic!("b"), not!(atomic!("a"))));
    }

    // !A & B | !A <=> !A
    #[test]
    fn test_absorption_law_double_not() {
        let expression = or!(and!(not!(atomic!("a")), atomic!("b")), not!(atomic!("a"))).absorption_law();
        assert_eq!(expression, not!(atomic!("a")));
    }

    // (A | B) & !A <=> B & !A
    #[test]
    fn test_in_parenthesis() {
        let expression = and!(or!(atomic!("a"), atomic!("b")), not!(atomic!("a"))).absorption_law();
        assert_eq!(expression, and!(atomic!("b"), not!(atomic!("a"))));
    }

    #[test]
    fn test_distributive_law_and() {
        let expression = and!(atomic!("a"), or!(atomic!("b"), atomic!("c"))).distribution_law();
        assert_eq!(expression, or!(and!(atomic!("a"), atomic!("b")), and!(atomic!("a"), atomic!("c"))));
    }

    #[test]
    fn test_distributive_law_or() {
        let expression = or!(atomic!("a"), and!(atomic!("b"), atomic!("c"))).distribution_law();
        assert_eq!(expression, and!(or!(atomic!("a"), atomic!("b")), or!(atomic!("a"), atomic!("c"))));
    }

    #[test]
    fn test_distributive_law_nested_not() {
        let expression = and!(atomic!("a"), not!(or!(atomic!("b"), atomic!("c")))).distribution_law();
        assert_eq!(expression, and!(atomic!("a"), not!(or!(atomic!("b"), atomic!("c")))))
    }
}
