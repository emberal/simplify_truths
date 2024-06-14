use serde::Serialize;

use crate::expressions::expression::{Expression, OppositeEq};
use crate::expressions::operator::BinaryOperator;
use crate::routing::response::Operation;

pub trait Simplify {
    fn elimination_of_implication(&self, operations: &mut Vec<Operation>) -> Self;
    fn double_negation_elimination(&self, operations: &mut Vec<Operation>) -> Self;
    fn de_morgans_laws(&self, operations: &mut Vec<Operation>) -> Self;
    fn absorption_law(&self, operations: &mut Vec<Operation>) -> Self;
    fn associative_law(&self, operations: &mut Vec<Operation>) -> Self;
    fn distribution_law(&self, operations: &mut Vec<Operation>) -> Self;
    fn commutative_law(&self, operations: &mut Vec<Operation>) -> Self;
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Law {
    EliminationOfImplication,
    DeMorgansLaws,
    AbsorptionLaw,
    AssociativeLaw,
    DistributionLaw,
    DoubleNegationElimination,
    CommutativeLaw,
}

impl Expression {
    // TODO for consecutive operations, use the previous result as the new before expression
    pub fn simplify(&self) -> (Self, Vec<Operation>) {
        let mut operations: Vec<Operation> = vec![];
        let expression = self.elimination_of_implication(&mut operations)
            .de_morgans_laws(&mut operations)
            .absorption_law(&mut operations)
            // .associative_law(&mut operations)
            .distribution_law(&mut operations)
            .double_negation_elimination(&mut operations);
        // .commutative_law(&mut operations);
        (expression, operations)
    }
}

impl Simplify for Expression {
    /// Eliminate the implication operator from the expression.
    /// This is done by replacing `a ➔ b` with `¬a ⋁ b`.
    fn elimination_of_implication(&self, operations: &mut Vec<Operation>) -> Self {
        let (before, after) = match self {
            not @ Expression::Not(expr) => (not.clone(), not!(expr.elimination_of_implication(operations))),
            Expression::Binary { left, operator, right } => {
                let l_result = left.elimination_of_implication(operations);
                let r_result = right.elimination_of_implication(operations);

                let before = binary!(l_result.clone(), *operator, r_result.clone());

                (before, if let BinaryOperator::Implication = *operator {
                    or!(not!(l_result), r_result)
                } else {
                    binary!(l_result, *operator, r_result)
                })
            }
            atomic @ Expression::Atomic(_) => (atomic.clone(), atomic.clone()),
        };
        if let Some(operation) = Operation::new(&before, &after, Law::EliminationOfImplication) {
            operations.push(operation);
        }
        after
    }

    /// Eliminate double negations from the expression.
    /// This is done by replacing `¬¬a` with `a`.
    /// This function is recursive and will continue to eliminate double negations until none are left.
    fn double_negation_elimination(&self, operations: &mut Vec<Operation>) -> Self {
        let result = match self {
            Expression::Not(expr) => {
                if let Expression::Not(inner) = *expr.clone() {
                    inner.double_negation_elimination(operations)
                } else {
                    not!(expr.double_negation_elimination(operations))
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.double_negation_elimination(operations);
                let right = right.double_negation_elimination(operations);
                binary!(left, *operator, right)
            }
            atomic @ Expression::Atomic(_) => atomic.clone(),
        };
        if let Some(operation) = Operation::new(self, &result, Law::DoubleNegationElimination) {
            operations.push(operation);
        }
        result
    }

    fn de_morgans_laws(&self, operations: &mut Vec<Operation>) -> Self {
        let result = match self {
            Expression::Not(expr) => {
                match *expr.clone() {
                    Expression::Binary { left, operator: BinaryOperator::And, right } => {
                        // TODO unnecessary cloning calls to de_morgans_laws?
                        let left = not!(left.de_morgans_laws(operations));
                        let right = not!(right.de_morgans_laws(operations));
                        or!(left, right).de_morgans_laws(operations)
                    }
                    Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                        let left = not!(left.de_morgans_laws(operations));
                        let right = not!(right.de_morgans_laws(operations));
                        and!(left, right).de_morgans_laws(operations)
                    }
                    _ => not!(expr.de_morgans_laws(operations)),
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.de_morgans_laws(operations);
                let right = right.de_morgans_laws(operations);
                binary!(left, *operator, right)
            }
            atomic @ Expression::Atomic(_) => atomic.clone(),
        };
        if let Some(operation) = Operation::new(self, &result, Law::DeMorgansLaws) {
            operations.push(operation);
        }
        result
    }

    // TODO deduplicate code
    fn absorption_law(&self, operations: &mut Vec<Operation>) -> Self {
        let result = match self {
            Expression::Binary { left, operator: BinaryOperator::And, right } => {
                let (left_ref, right_ref) = (left.as_ref(), right.as_ref());
                match (left_ref, right_ref) {
                    (_, Expression::Binary { left: right_left, operator: BinaryOperator::Or, right: right_right }) => {
                        if left_ref == right_left.as_ref() || left_ref == right_right.as_ref() {
                            return left.absorption_law(operations);
                        } else if right_left.is_atomic() && right_right.is_atomic() && left.opposite_eq(right_left) {
                            if left.opposite_eq(right_left) {
                                return and!(left.absorption_law(operations), right_left.absorption_law(operations));
                            } else if left.opposite_eq(right_right) {
                                return and!(left.absorption_law(operations), right_right.absorption_law(operations));
                            }
                        }
                        and!(left.absorption_law(operations), right.absorption_law(operations))
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::Or, right: left_right }, _) => {
                        if right_ref == left_left.as_ref() || right_ref == left_right.as_ref() {
                            return right.absorption_law(operations);
                        } else if left_left.is_atomic() && left_right.is_atomic() && right.opposite_eq(left_left) {
                            if right.opposite_eq(left_left) {
                                return and!(left_right.absorption_law(operations), right.absorption_law(operations));
                            } else if right.opposite_eq(left_right) {
                                return and!(left_left.absorption_law(operations), right.absorption_law(operations));
                            }
                        }
                        and!(left.absorption_law(operations), right.absorption_law(operations))
                    }
                    (left, right) => and!(left.absorption_law(operations), right.absorption_law(operations))
                }
            }
            Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                let (left_ref, right_ref) = (left.as_ref(), right.as_ref());
                match (left_ref, right_ref) {
                    (_, Expression::Binary { left: right_left, operator: BinaryOperator::And, right: right_right }) => {
                        if left_ref == right_left.as_ref() || left_ref == right_right.as_ref() {
                            return left.absorption_law(operations);
                        } else if right_left.is_atomic() && right_right.is_atomic() && left.opposite_eq(right_left) {
                            if left.opposite_eq(right_left) {
                                return or!(left.absorption_law(operations), right_left.absorption_law(operations));
                            } else if left.opposite_eq(right_right) {
                                return or!(left.absorption_law(operations), right_right.absorption_law(operations));
                            }
                        }
                        or!(left.absorption_law(operations), right.absorption_law(operations))
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::And, right: left_right }, _) => {
                        if right_ref == left_left.as_ref() || right_ref == left_right.as_ref() {
                            return right.absorption_law(operations);
                        } else if left_left.is_atomic() && left_right.is_atomic() && right.opposite_eq(left_left) {
                            if right.opposite_eq(left_left) {
                                return or!(left_right.absorption_law(operations), right.absorption_law(operations));
                            } else if right.opposite_eq(left_right) {
                                return or!(left_left.absorption_law(operations), right.absorption_law(operations));
                            }
                        }
                        or!(left.absorption_law(operations), right.absorption_law(operations))
                    }
                    (left, right) => or!(left.absorption_law(operations), right.absorption_law(operations))
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.absorption_law(operations);
                let right = right.absorption_law(operations);
                binary!(left, *operator, right)
            }
            Expression::Not(expr) => not!(expr.absorption_law(operations)),
            atomic => atomic.clone(),
        };
        if let Some(operation) = Operation::new(self, &result, Law::AbsorptionLaw) {
            operations.push(operation);
        }
        result
    }

    fn associative_law(&self, operations: &mut Vec<Operation>) -> Self {
        todo!("? | Associative law: (a ⋀ b) ⋀ c == a ⋀ (b ⋀ c) and (a ⋁ b) ⋁ c == a ⋁ (b ⋁ c)")
    }

    // TODO deduplicate code
    fn distribution_law(&self, operations: &mut Vec<Operation>) -> Self {
        let result = match self {
            Expression::Binary { left, operator: BinaryOperator::And, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (Expression::Atomic(_), Expression::Binary { left: right_left, operator: BinaryOperator::Or, right: right_right }) => {
                        let right_left = right_left.distribution_law(operations);
                        let right_right = right_right.distribution_law(operations);
                        or!(and!(*left.clone(), right_left), and!(*left.clone(), right_right))
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::Or, right: left_right }, Expression::Atomic(_)) => {
                        let left_left = left_left.distribution_law(operations);
                        let left_right = left_right.distribution_law(operations);
                        or!(and!(left_left, *right.clone()), and!(left_right, *right.clone()))
                    }
                    (left, right) => and!(left.distribution_law(operations), right.distribution_law(operations))
                }
            }
            Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (Expression::Atomic(_), Expression::Binary { left: right_left, operator: BinaryOperator::And, right: right_right }) => {
                        let right_left = right_left.distribution_law(operations);
                        let right_right = right_right.distribution_law(operations);
                        and!(or!(*left.clone(), right_left), or!(*left.clone(), right_right))
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::And, right: left_right }, Expression::Atomic(_)) => {
                        let left_left = left_left.distribution_law(operations);
                        let left_right = left_right.distribution_law(operations);
                        and!(or!(left_left, *right.clone()), or!(left_right, *right.clone()))
                    }
                    (left, right) => or!(left.distribution_law(operations), right.distribution_law(operations))
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.distribution_law(operations);
                let right = right.distribution_law(operations);
                binary!(left, *operator, right)
            }
            Expression::Not(expr) => not!(expr.distribution_law(operations)),
            atomic => atomic.clone(),
        };
        if let Some(operation) = Operation::new(self, &result, Law::DistributionLaw) {
            operations.push(operation);
        }
        result
    }

    fn commutative_law(&self, operations: &mut Vec<Operation>) -> Self {
        todo!("? | Order of operands does not matter in AND and OR operations.")
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::simplify::{Law, Simplify};

    #[test]
    fn test_simplify() {
        let expression = eval!("a" => "b").simplify().0;
        assert_eq!(expression, or!(not!(atomic!("a")), atomic!("b")));
    }

    #[test]
    fn test_implication_and_de_morgans() {
        let expression = implies!(and!(not!(atomic!("a")), atomic!("b")), atomic!("c")).simplify().0;
        assert_eq!(expression, or!(or!(atomic!("a"), not!(atomic!("b"))), atomic!("c")));
    }

    #[test]
    fn test_elimination_of_implication() {
        let mut operations = vec![];
        let expression = eval!("a" => "b").elimination_of_implication(&mut operations);
        assert_eq!(expression, or!(not!(atomic!("a")), atomic!("b")));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::EliminationOfImplication);
        assert_eq!(operations[0].before, "a ➔ b");
        assert_eq!(operations[0].after, "(¬a ⋁ b)");
    }

    #[test]
    fn test_elimination_of_implication_nested() {
        let mut operations = vec![];
        let expression = implies!(atomic!("a"), implies!(atomic!("b"), atomic!("c"))).elimination_of_implication(&mut operations);
        assert_eq!(expression, or!(not!(atomic!("a")), or!(not!(atomic!("b")), atomic!("c"))));
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::EliminationOfImplication);
        assert_eq!(operations[0].before, "b ➔ c");
        assert_eq!(operations[0].after, "(¬b ⋁ c)");
        assert_eq!(operations[1].law, Law::EliminationOfImplication);
        assert_eq!(operations[1].before, "a ➔ (¬b ⋁ c)");
        assert_eq!(operations[1].after, "(¬a ⋁ (¬b ⋁ c))");
    }

    #[test]
    fn test_elimination_of_implication_none() {
        let mut operations = vec![];
        let expression = eval!("a" && "b").elimination_of_implication(&mut operations);
        assert_eq!(expression, eval!("a" && "b"));
        assert_eq!(operations.len(), 0);
    }

    #[test]
    fn test_elimination_of_implication_nested_none() {
        let mut operations = vec![];
        let expression = or!(atomic!("a"), and!(atomic!("b"), atomic!("c"))).elimination_of_implication(&mut operations);
        assert_eq!(expression, or!(atomic!("a"), and!(atomic!("b"), atomic!("c"))));
        assert_eq!(operations.len(), 0);
    }

    #[test]
    fn test_double_negation_elimination() {
        let mut operations = vec![];
        let expression = not!(not!(atomic!("a"))).double_negation_elimination(&mut operations);
        assert_eq!(expression, atomic!("a"));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DoubleNegationElimination);
        assert_eq!(operations[0].before, "¬¬a");
        assert_eq!(operations[0].after, "a");
    }

    #[test]
    fn test_triple_negation_elimination() {
        let mut operations = vec![];
        let expression = not!(not!(not!(atomic!("a")))).double_negation_elimination(&mut operations);
        assert_eq!(expression, not!(atomic!("a")));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DoubleNegationElimination);
        assert_eq!(operations[0].before, "¬¬¬a");
        assert_eq!(operations[0].after, "¬a");
    }

    #[test]
    fn test_five_negation_elimination() {
        let mut operations = vec![];
        let expression = not!(not!(not!(not!(not!(atomic!("a")))))).double_negation_elimination(&mut operations);
        assert_eq!(expression, not!(atomic!("a")));
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::DoubleNegationElimination);
        assert_eq!(operations[0].before, "¬¬¬a");
        assert_eq!(operations[0].after, "¬a");
        assert_eq!(operations[1].law, Law::DoubleNegationElimination);
        assert_eq!(operations[1].before, "¬¬¬¬¬a");
        assert_eq!(operations[1].after, "¬¬¬a");
    }

    #[test]
    fn test_no_negation_elimination() {
        let mut operations = vec![];
        let expression = atomic!("a").double_negation_elimination(&mut operations);
        assert_eq!(expression, atomic!("a"));
        assert_eq!(operations.len(), 0);
    }

    #[test]
    fn test_double_negation_nested_elimination() {
        let mut operations = vec![];
        let expression = and!(or!(not!(eval!(!"a")), eval!("b")), not!(eval!(!"c"))).double_negation_elimination(&mut operations);
        assert_eq!(expression, and!(or!(atomic!("a"), atomic!("b")), atomic!("c")));
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::DoubleNegationElimination);
        assert_eq!(operations[0].before, "¬¬a");
        assert_eq!(operations[0].after, "a");
        assert_eq!(operations[1].law, Law::DoubleNegationElimination);
        assert_eq!(operations[1].before, "¬¬c");
        assert_eq!(operations[1].after, "c");
    }

    #[test]
    fn test_de_morgans_laws_and() {
        let mut operations = vec![];
        let expression = not!(eval!("a" && "b")).de_morgans_laws(&mut operations);
        assert_eq!(expression, or!(not!(atomic!("a")), not!(atomic!("b"))));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DeMorgansLaws);
        assert_eq!(operations[0].before, "¬(a ⋀ b)");
        assert_eq!(operations[0].after, "(¬a ⋁ ¬b)");
    }

    #[test]
    fn test_de_morgans_laws_or() {
        let mut operations = vec![];
        let expression = not!(eval!("a" || "b")).de_morgans_laws(&mut operations);
        assert_eq!(expression, and!(not!(atomic!("a")), not!(atomic!("b"))));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DeMorgansLaws);
        assert_eq!(operations[0].before, "¬((a ⋁ b))");
        assert_eq!(operations[0].after, "¬a ⋀ ¬b");
    }

    #[test]
    fn test_de_morgans_laws_nested_or() {
        let mut operations = vec![];
        let expression = not!(or!(eval!("a" && "b"), atomic!("c"))).de_morgans_laws(&mut operations); // ¬(a ⋀ b ⋁ c)
        assert_eq!(expression, and!(or!(eval!(!"a"), eval!(!"b")), eval!(!"c"))); // ¬(a ⋀ b) ⋀ ¬c == (¬a ⋁ ¬b) ⋀ ¬c
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::DeMorgansLaws);
        assert_eq!(operations[0].before, "¬(a ⋀ b)");
        assert_eq!(operations[0].after, "(¬a ⋁ ¬b)");
        assert_eq!(operations[1].law, Law::DeMorgansLaws);
        assert_eq!(operations[1].before, "¬(a ⋀ b ⋁ c)");
        assert_eq!(operations[1].after, "¬(a ⋀ b) ⋀ ¬c");
    }

    #[test]
    fn test_de_morgans_laws_nested_and() {
        let mut operations = vec![];
        let expression = not!(and!(eval!("a" || "b"), atomic!("c"))).de_morgans_laws(&mut operations); // ¬(a ⋁ b ⋀ c)
        assert_eq!(expression, or!(and!(eval!(!"a"), eval!(!"b")), eval!(!"c"))); // ¬(a ⋁ b) ⋀ ¬c == (¬a ⋀ ¬b) ⋁ ¬c
    }

    #[test]
    fn test_de_morgans_laws_nested_and_or() {
        let mut operations = vec![];
        let expression = not!(and!(eval!("a" || "b"), or!(atomic!("c"), atomic!("d")))).de_morgans_laws(&mut operations); // ¬(a ⋁ b ⋀ c ⋁ d)
        assert_eq!(expression, or!(and!(eval!(!"a"), eval!(!"b")), and!(eval!(!"c"), eval!(!"d")))); // ¬(a ⋁ b) ⋀ ¬(c ⋁ d) == (¬a ⋀ ¬b) ⋁ (¬c ⋀ ¬d)
    }

    #[test]
    fn test_absorption_law_and() {
        let mut operations = vec![];
        let expression = and!(atomic!("a"), eval!("a" || "b")).absorption_law(&mut operations);
        assert_eq!(expression, atomic!("a"));
    }

    #[test]
    fn test_absorption_law_or() {
        let mut operations = vec![];
        let expression = or!(atomic!("a"), eval!("a" && "b")).absorption_law(&mut operations);
        assert_eq!(expression, atomic!("a"));
    }

    #[test]
    fn test_absorption_law_nested_and() {
        let mut operations = vec![];
        let expression = and!(atomic!("a"), or!(atomic!("a"), atomic!("b"))).absorption_law(&mut operations);
        assert_eq!(expression, atomic!("a"));
    }

    // !A & B | A <=> B | A
    #[test]
    fn test_absorption_law_not() {
        let mut operations = vec![];
        let expression = or!(and!(not!(atomic!("a")), atomic!("b")), atomic!("a")).absorption_law(&mut operations);
        assert_eq!(expression, or!(atomic!("b"), atomic!("a")));
    }

    // A & B | !A <=> B | !A
    #[test]
    fn test_absorption_law_not_reversed() {
        let mut operations = vec![];
        let expression = or!(and!(atomic!("a"), atomic!("b")), not!(atomic!("a"))).absorption_law(&mut operations);
        assert_eq!(expression, or!(atomic!("b"), not!(atomic!("a"))));
    }

    // !A & B | !A <=> !A
    #[test]
    fn test_absorption_law_double_not() {
        let mut operations = vec![];
        let expression = or!(and!(not!(atomic!("a")), atomic!("b")), not!(atomic!("a"))).absorption_law(&mut operations);
        assert_eq!(expression, not!(atomic!("a")));
    }

    // (A | B) & !A <=> B & !A
    #[test]
    fn test_in_parenthesis() {
        let mut operations = vec![];
        let expression = and!(or!(atomic!("a"), atomic!("b")), not!(atomic!("a"))).absorption_law(&mut operations);
        assert_eq!(expression, and!(atomic!("b"), not!(atomic!("a"))));
    }

    #[test]
    fn test_distributive_law_and() {
        let mut operations = vec![];
        let expression = and!(atomic!("a"), or!(atomic!("b"), atomic!("c"))).distribution_law(&mut operations);
        assert_eq!(expression, or!(and!(atomic!("a"), atomic!("b")), and!(atomic!("a"), atomic!("c"))));
    }

    #[test]
    fn test_distributive_law_or() {
        let mut operations = vec![];
        let expression = or!(atomic!("a"), and!(atomic!("b"), atomic!("c"))).distribution_law(&mut operations);
        assert_eq!(expression, and!(or!(atomic!("a"), atomic!("b")), or!(atomic!("a"), atomic!("c"))));
    }

    #[test]
    fn test_distributive_law_nested_not() {
        let mut operations = vec![];
        let expression = and!(atomic!("a"), not!(or!(atomic!("b"), atomic!("c")))).distribution_law(&mut operations);
        assert_eq!(expression, and!(atomic!("a"), not!(or!(atomic!("b"), atomic!("c")))))
    }
}
