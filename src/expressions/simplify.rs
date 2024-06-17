use std::ops::Deref;

use serde::Serialize;

use crate::expressions::expression::{Expression, OppositeEq};
use crate::expressions::helpers::{and, binary, not, or};
use crate::expressions::operator::BinaryOperator;
use crate::routing::response::Operation;

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

// TODO deduplicate code
impl Expression {
    // TODO better track of operations
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

    /// Eliminate the implication operator from the expression.
    /// This is done by replacing `a ➔ b` with `¬a ⋁ b`.
    fn elimination_of_implication(&self, operations: &mut Vec<Operation>) -> Self {
        let result = match self {
            Expression::Not(expr) => not(expr.elimination_of_implication(operations)),
            Expression::Binary { left, operator, right } => {
                let l_result = left.elimination_of_implication(operations);
                let r_result = right.elimination_of_implication(operations);

                if let BinaryOperator::Implication = *operator {
                    or(not(l_result), r_result)
                } else {
                    binary(l_result, *operator, r_result)
                }
            }
            atomic @ Expression::Atomic(_) => atomic.clone(),
        };
        if let Some(operation) = Operation::new(self, &result, Law::EliminationOfImplication) {
            operations.push(operation);
        }
        result
    }

    /// Eliminate double negations from the expression.
    /// This is done by replacing `¬¬a` with `a`.
    /// This function is recursive and will continue to eliminate double negations until none are left.
    fn double_negation_elimination(&self, operations: &mut Vec<Operation>) -> Self {
        let result = match self {
            Expression::Not(expr) => {
                if let Expression::Not(inner) = expr.deref() {
                    inner.double_negation_elimination(operations)
                } else {
                    not(expr.double_negation_elimination(operations))
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.double_negation_elimination(operations);
                let right = right.double_negation_elimination(operations);
                binary(left.clone(), *operator, right.clone())
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
                match expr.deref() {
                    Expression::Binary { left, operator: BinaryOperator::And, right } => {
                        let left = not(left.de_morgans_laws(operations));
                        let right = not(right.de_morgans_laws(operations));
                        or(left, right).de_morgans_laws(operations)
                    }
                    Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                        let left = not(left.de_morgans_laws(operations));
                        let right = not(right.de_morgans_laws(operations));
                        and(left, right).de_morgans_laws(operations)
                    }
                    _ => not(expr.de_morgans_laws(operations)),
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.de_morgans_laws(operations);
                let right = right.de_morgans_laws(operations);
                binary(left, *operator, right)
            }
            atomic @ Expression::Atomic(_) => atomic.clone(),
        };
        if let Some(operation) = Operation::new(self, &result, Law::DeMorgansLaws) {
            operations.push(operation);
        }
        result
    }

    fn absorption_law(&self, operations: &mut Vec<Operation>) -> Self {
        let result = match self {
            Expression::Binary { left, operator: BinaryOperator::And | BinaryOperator::Or, right } if *left == *right => {
                left.absorption_law(operations)
            }
            Expression::Binary { left, operator: BinaryOperator::And, right } => {
                let (left_ref, right_ref) = (left.as_ref(), right.as_ref());
                match (left_ref, right_ref) {
                    (_, Expression::Binary { left: right_left, operator: BinaryOperator::Or, right: right_right }) => {
                        evaluate_equals_or_opposites(left_ref, right_left, right_right, and, operations).unwrap_or(
                            and(left.absorption_law(operations), right.absorption_law(operations))
                        )
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::Or, right: left_right }, _) => {
                        evaluate_equals_or_opposites(right_ref, left_left, left_right, and, operations).unwrap_or(
                            and(left.absorption_law(operations), right.absorption_law(operations))
                        )
                    }
                    (left, right) => and(left.absorption_law(operations), right.absorption_law(operations))
                }
            }
            Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (_, Expression::Binary { left: right_left, operator: BinaryOperator::And, right: right_right }) => {
                        evaluate_equals_or_opposites(left.as_ref(), right_left, right_right, or, operations).unwrap_or(
                            or(left.absorption_law(operations), right.absorption_law(operations))
                        )
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::And, right: left_right }, _) => {
                        evaluate_equals_or_opposites(right.as_ref(), left_left, left_right, or, operations).unwrap_or(
                            or(left.absorption_law(operations), right.absorption_law(operations))
                        )
                    }
                    (left, right) => or(left.absorption_law(operations), right.absorption_law(operations))
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.absorption_law(operations);
                let right = right.absorption_law(operations);
                binary(left, *operator, right)
            }
            Expression::Not(expr) => not(expr.absorption_law(operations)),
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

    fn distribution_law(&self, operations: &mut Vec<Operation>) -> Self {
        let result = match self {
            Expression::Binary { left, operator: BinaryOperator::And, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (Expression::Atomic(_), Expression::Binary { left: right_left, operator: BinaryOperator::Or, right: right_right }) => {
                        let right_left = right_left.distribution_law(operations);
                        let right_right = right_right.distribution_law(operations);
                        or(and(left.clone(), right_left), and(left.clone(), right_right))
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::Or, right: left_right }, Expression::Atomic(_)) => {
                        let left_left = left_left.distribution_law(operations);
                        let left_right = left_right.distribution_law(operations);
                        or(and(left_left, right.clone()), and(left_right, right.clone()))
                    }
                    (left, right) => and(left.distribution_law(operations), right.distribution_law(operations))
                }
            }
            Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (Expression::Atomic(_), Expression::Binary { left: right_left, operator: BinaryOperator::And, right: right_right }) => {
                        let right_left = right_left.distribution_law(operations);
                        let right_right = right_right.distribution_law(operations);
                        and(or(left.clone(), right_left), or(left.clone(), right_right))
                    }
                    (Expression::Binary { left: left_left, operator: BinaryOperator::And, right: left_right }, Expression::Atomic(_)) => {
                        let left_left = left_left.distribution_law(operations);
                        let left_right = left_right.distribution_law(operations);
                        and(or(left_left, right.clone()), or(left_right, right.clone()))
                    }
                    (left, right) => or(left.distribution_law(operations), right.distribution_law(operations))
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.distribution_law(operations);
                let right = right.distribution_law(operations);
                binary(left, *operator, right)
            }
            Expression::Not(expr) => not(expr.distribution_law(operations)),
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

fn evaluate_equals_or_opposites<F: Fn(Expression, Expression) -> Expression>(
    this: &Expression,
    left: &Expression,
    right: &Expression,
    ret_func: F,
    operations: &mut Vec<Operation>,
) -> Option<Expression> {
    if *this == *left || *this == *right {
        return Some(this.absorption_law(operations));
    } else if left.is_atomic() && right.is_atomic() && this.opposite_eq(left) {
        if this.opposite_eq(left) {
            return Some(ret_func(right.absorption_law(operations), this.absorption_law(operations)));
        } else if this.opposite_eq(right) {
            return Some(ret_func(left.absorption_law(operations), this.absorption_law(operations)));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::expressions::helpers::{and, atomic, implies, not, or};
    use crate::expressions::simplify::Law;

    #[test]
    fn test_simplify() {
        let (expression, operations) = implies(atomic("a"), atomic("b")).simplify();
        assert_eq!(expression, or(not(atomic("a")), atomic("b")));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::EliminationOfImplication);
    }

    #[test]
    fn test_simplify_a_and_a() {
        let (expression, operations) = and(atomic("a"), atomic("a")).simplify();
        assert_eq!(expression, atomic("a"));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::AbsorptionLaw);
    }

    #[test]
    fn test_implication_and_de_morgans() {
        let expression = implies(and(not(atomic("a")), atomic("b")), atomic("c")).simplify().0;
        assert_eq!(expression, or(or(atomic("a"), not(atomic("b"))), atomic("c")));
    }

    #[test]
    fn test_elimination_of_implication() {
        let mut operations = vec![];
        let expression = implies(atomic("a"), atomic("b")).elimination_of_implication(&mut operations);
        assert_eq!(expression, or(not(atomic("a")), atomic("b")));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::EliminationOfImplication);
        assert_eq!(operations[0].before, "a ➔ b");
        assert_eq!(operations[0].after, "¬a ⋁ b");
    }

    #[test]
    fn test_elimination_of_implication_nested() {
        let mut operations = vec![];
        let expression = implies(atomic("a"), implies(atomic("b"), atomic("c"))).elimination_of_implication(&mut operations);
        assert_eq!(expression, or(not(atomic("a")), or(not(atomic("b")), atomic("c"))));
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::EliminationOfImplication);
        assert_eq!(operations[0].before, "b ➔ c");
        assert_eq!(operations[0].after, "¬b ⋁ c");
        assert_eq!(operations[1].law, Law::EliminationOfImplication);
        assert_eq!(operations[1].before, "a ➔ b ➔ c");
        assert_eq!(operations[1].after, "¬a ⋁ ¬b ⋁ c");
    }

    #[test]
    fn test_elimination_of_implication_none() {
        let mut operations = vec![];
        let expression = and(atomic("a"), atomic("b")).elimination_of_implication(&mut operations);
        assert_eq!(expression, and(atomic("a"), atomic("b")));
    }

    #[test]
    fn test_elimination_of_implication_nested_none() {
        let mut operations = vec![];
        let expression = or(atomic("a"), and(atomic("b"), atomic("c"))).elimination_of_implication(&mut operations);
        assert_eq!(expression, or(atomic("a"), and(atomic("b"), atomic("c"))));
    }

    #[test]
    fn test_double_negation_elimination() {
        let mut operations = vec![];
        let expression = not(not(atomic("a"))).double_negation_elimination(&mut operations);
        assert_eq!(expression, atomic("a"));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DoubleNegationElimination);
        assert_eq!(operations[0].before, "¬¬a");
        assert_eq!(operations[0].after, "a");
    }

    #[test]
    fn test_triple_negation_elimination() {
        let mut operations = vec![];
        let expression = not(not(not(atomic("a")))).double_negation_elimination(&mut operations);
        assert_eq!(expression, not(atomic("a")));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DoubleNegationElimination);
        assert_eq!(operations[0].before, "¬¬¬a");
        assert_eq!(operations[0].after, "¬a");
    }

    #[test]
    fn test_five_negation_elimination() {
        let mut operations = vec![];
        let expression = not(not(not(not(not(atomic("a")))))).double_negation_elimination(&mut operations);
        assert_eq!(expression, not(atomic("a")));
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::DoubleNegationElimination);
        assert_eq!(operations[0].before, "¬¬¬a");
        assert_eq!(operations[0].after, "¬a");
        assert_eq!(operations[1].law, Law::DoubleNegationElimination);
        assert_eq!(operations[1].before, "¬¬¬¬¬a");
        assert_eq!(operations[1].after, "¬a");
    }

    #[test]
    fn test_no_negation_elimination() {
        let mut operations = vec![];
        let expression = atomic("a").double_negation_elimination(&mut operations);
        assert_eq!(expression, atomic("a"));
    }

    #[test]
    fn test_double_negation_nested_elimination() {
        let mut operations = vec![];
        let expression = and(or(not(not(atomic("a"))), atomic("b")), not(not(atomic("c")))).double_negation_elimination(&mut operations);
        assert_eq!(expression, and(or(atomic("a"), atomic("b")), atomic("c")));
        assert_eq!(operations.len(), 4);
        assert!(operations.into_iter().map(|operation| operation.law).all(|law| law == Law::DoubleNegationElimination));
    }

    #[test]
    fn test_de_morgans_laws_and() {
        let mut operations = vec![];
        let expression = not(and(atomic("a"), atomic("b"))).de_morgans_laws(&mut operations);
        assert_eq!(expression, or(not(atomic("a")), not(atomic("b"))));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DeMorgansLaws);
        assert_eq!(operations[0].before, "¬(a ⋀ b)");
        assert_eq!(operations[0].after, "¬a ⋁ ¬b");
    }

    #[test]
    fn test_de_morgans_laws_or() {
        let mut operations = vec![];
        let expression = not(or(atomic("a"), atomic("b"))).de_morgans_laws(&mut operations);
        assert_eq!(expression, and(not(atomic("a")), not(atomic("b"))));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DeMorgansLaws);
        assert_eq!(operations[0].before, "¬((a ⋁ b))");
        assert_eq!(operations[0].after, "¬a ⋀ ¬b");
    }

    #[test]
    fn test_de_morgans_laws_nested_or() {
        let mut operations = vec![];
        let expression = not(or(and(atomic("a"), atomic("b")), atomic("c"))).de_morgans_laws(&mut operations); // ¬(a ⋀ b ⋁ c)
        assert_eq!(expression, and(or(not(atomic("a")), not(atomic("b"))), not(atomic("c")))); // ¬(a ⋀ b) ⋀ ¬c == (¬a ⋁ ¬b) ⋀ ¬c
        assert_eq!(operations.len(), 3);
        assert!(operations.into_iter().map(|operation| operation.law).all(|law| law == Law::DeMorgansLaws));
    }

    #[test]
    fn test_de_morgans_laws_nested_and() {
        let mut operations = vec![];
        let expression = not(and(or(atomic("a"), atomic("b")), atomic("c"))).de_morgans_laws(&mut operations); // ¬(a ⋁ b ⋀ c)
        assert_eq!(expression, or(and(not(atomic("a")), not(atomic("b"))), not(atomic("c")))); // ¬(a ⋁ b) ⋀ ¬c == (¬a ⋀ ¬b) ⋁ ¬c
    }

    #[test]
    fn test_de_morgans_laws_nested_and_or() {
        let mut operations = vec![];
        let expression = not(and(or(atomic("a"), atomic("b")), or(atomic("c"), atomic("d")))).de_morgans_laws(&mut operations); // ¬(a ⋁ b ⋀ c ⋁ d)
        assert_eq!(expression, or(and(not(atomic("a")), not(atomic("b"))), and(not(atomic("c")), not(atomic("d"))))); // ¬(a ⋁ b) ⋀ ¬(c ⋁ d) == (¬a ⋀ ¬b) ⋁ (¬c ⋀ ¬d)
    }

    #[test]
    fn test_absorption_law_and() {
        let mut operations = vec![];
        let expression = and(atomic("a"), or(atomic("a"), atomic("b"))).absorption_law(&mut operations);
        assert_eq!(expression, atomic("a"));
    }

    #[test]
    fn test_absorption_law_or() {
        let mut operations = vec![];
        let expression = or(atomic("a"), and(atomic("a"), atomic("b"))).absorption_law(&mut operations);
        assert_eq!(expression, atomic("a"));
    }

    #[test]
    fn test_absorption_law_nested_and() {
        let mut operations = vec![];
        let expression = and(atomic("a"), or(atomic("a"), atomic("b"))).absorption_law(&mut operations);
        assert_eq!(expression, atomic("a"));
    }

    // !A & B | A <=> B | A
    #[test]
    fn test_absorption_law_not() {
        let mut operations = vec![];
        let expression = or(and(not(atomic("a")), atomic("b")), atomic("a")).absorption_law(&mut operations);
        assert_eq!(expression, or(atomic("b"), atomic("a")));
    }

    // A & B | !A <=> B | !A
    #[test]
    fn test_absorption_law_not_reversed() {
        let mut operations = vec![];
        let expression = or(and(atomic("a"), atomic("b")), not(atomic("a"))).absorption_law(&mut operations);
        assert_eq!(expression, or(atomic("b"), not(atomic("a"))));
    }

    // !A & B | !A <=> !A
    #[test]
    fn test_absorption_law_double_not() {
        let mut operations = vec![];
        let expression = or(and(not(atomic("a")), atomic("b")), not(atomic("a"))).absorption_law(&mut operations);
        assert_eq!(expression, not(atomic("a")));
    }

    #[test]
    fn test_absorption_law_duplicate_atomic() {
        let mut operations = vec![];
        let expression = and(atomic("A"), atomic("A"));
        let simplified = expression.absorption_law(&mut operations);
        assert_eq!(simplified, atomic("A"));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::AbsorptionLaw);
        assert_eq!(operations[0].before, "A ⋀ A");
        assert_eq!(operations[0].after, "A");
    }

    // (A | B) & !A <=> B & !A
    #[test]
    fn test_in_parenthesis() {
        let mut operations = vec![];
        let expression = and(or(atomic("a"), atomic("b")), not(atomic("a"))).absorption_law(&mut operations);
        assert_eq!(expression, and(atomic("b"), not(atomic("a"))));
    }

    #[test]
    fn test_distributive_law_and() {
        let mut operations = vec![];
        let expression = and(atomic("a"), or(atomic("b"), atomic("c"))).distribution_law(&mut operations);
        assert_eq!(expression, or(and(atomic("a"), atomic("b")), and(atomic("a"), atomic("c"))));
    }

    #[test]
    fn test_distributive_law_or() {
        let mut operations = vec![];
        let expression = or(atomic("a"), and(atomic("b"), atomic("c"))).distribution_law(&mut operations);
        assert_eq!(expression, and(or(atomic("a"), atomic("b")), or(atomic("a"), atomic("c"))));
    }

    #[test]
    fn test_distributive_law_nested_not() {
        let mut operations = vec![];
        let expression = and(atomic("a"), not(or(atomic("b"), atomic("c")))).distribution_law(&mut operations);
        assert_eq!(expression, and(atomic("a"), not(or(atomic("b"), atomic("c")))))
    }
}
