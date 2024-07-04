use std::ops::Deref;

use serde::Serialize;

use crate::expressions::expression::Expression;
use crate::expressions::helpers::{and, binary, not, or};
use crate::expressions::operator::BinaryOperator;
use crate::routing::options::SimplifyOptions;
use crate::routing::response::Operation;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum Law {
    EliminationOfImplication,
    DeMorgansLaws,
    AbsorptionLaw,
    AssociativeLaw,
    DistributiveLaw,
    DoubleNegationElimination,
    CommutativeLaw,
}

// TODO refactor
#[macro_export]
macro_rules! absorption_law_opposites {
    ($left:expr, $right:expr, $operations:expr, $this_op:pat, $op:pat, $func:expr, $ignore_case:expr) => {
        {
            let before = $func($left.clone(), $right.clone());
            match ($left.as_ref(), $right.as_ref()) {
                (_, Expression::Binary { left: right_left, operator: $op, right: right_right }) => {
                    let result = evaluate_equals_or_opposites($left.as_ref(), right_left, right_right, $func, $ignore_case, $operations).unwrap_or(
                        $func($left.absorption_law($operations, $ignore_case), $right.absorption_law($operations, $ignore_case))
                    );
                    if let Some(operation) = Operation::new(&before, &result, Law::AbsorptionLaw) {
                        $operations.push(operation);
                    }
                    result
                }
                (_, Expression::Binary { left: right_left, operator: $this_op, .. })
                if $left.opposite_eq(right_left, $ignore_case) => {
                    let result = $func($left.clone(), right_left.clone());
                    if let Some(operation) = Operation::new(&before, &result, Law::AbsorptionLaw) {
                        $operations.push(operation);
                    }
                    result
                }
                (_, Expression::Binary { right: right_right, operator: $this_op, .. })
                if $left.opposite_eq(right_right, $ignore_case) => {
                    let result = $func($left.clone(), right_right.clone());
                    if let Some(operation) = Operation::new(&before, &result, Law::AbsorptionLaw) {
                        $operations.push(operation);
                    }
                    result
                }
                (Expression::Binary { left: left_left, operator: $op, right: left_right }, _) => {
                    let result = evaluate_equals_or_opposites($right.as_ref(), left_left, left_right, $func, $ignore_case, $operations).unwrap_or(
                        $func($left.absorption_law($operations, $ignore_case), $right.absorption_law($operations, $ignore_case))
                    );
                    if let Some(operation) = Operation::new(&before, &result, Law::AbsorptionLaw) {
                        $operations.push(operation);
                    }
                    result
                }
                (Expression::Binary { left: left_left, operator: $this_op, .. }, _)
                if $right.opposite_eq(left_left, $ignore_case) => {
                    let result = $func($right.clone(), left_left.clone());
                    if let Some(operation) = Operation::new(&before, &result, Law::AbsorptionLaw) {
                        $operations.push(operation);
                    }
                    result
                }
                (Expression::Binary { right: left_right, operator: $this_op, .. }, _)
                if $right.opposite_eq(left_right, $ignore_case) => {
                    let result = $func($right.clone(), left_right.clone());
                    if let Some(operation) = Operation::new(&before, &result, Law::AbsorptionLaw) {
                        $operations.push(operation);
                    }
                    result
                }
                (left, right) => $func(left.absorption_law($operations, $ignore_case), right.absorption_law($operations, $ignore_case))
            }
        }
    };
}

#[macro_export]
macro_rules! distributive_law_atomic_vs_binary {
    ($left:expr, $right:expr, $operations:expr, $op:pat, $func1:expr, $func2:expr) => {
        match ($left.as_ref(), $right.as_ref()) {
            (Expression::Atomic(_), Expression::Binary { left: right_left, operator: $op, right: right_right }) => {
                let right_left = right_left.distributive_law($operations);
                let right_right = right_right.distributive_law($operations);
                let before = $func2($left.clone(), $right.clone());
                let after = $func1($func2($left.clone(), right_left), $func2($left.clone(), right_right));
                if let Some(operation) = Operation::new(&before, &after, Law::DistributiveLaw) {
                    $operations.push(operation);
                }
                after
            }
            (Expression::Binary { left: left_left, operator: $op, right: left_right }, Expression::Atomic(_)) => {
                let left_left = left_left.distributive_law($operations);
                let left_right = left_right.distributive_law($operations);
                let before = $func2($left.clone(), $right.clone());
                let after = $func1($func2(left_left, $right.clone()), $func2(left_right, $right.clone()));
                if let Some(operation) = Operation::new(&before, &after, Law::DistributiveLaw) {
                    $operations.push(operation);
                }
                after
            }
            (left, right) => $func2(left.distributive_law($operations), right.distributive_law($operations))
        }
    };
}

#[derive(Debug, Default)]
pub struct Options {
    pub ignore_case: bool,
}

impl From<SimplifyOptions> for Options {
    fn from(options: SimplifyOptions) -> Self {
        Self { ignore_case: options.ignore_case }
    }
}

// TODO refactor, remove unnecessary code and split up into smaller functions
impl Expression {
    // TODO better track of operations
    pub fn simplify(&self, options: Options) -> (Self, Vec<Operation>) {
        let mut operations: Vec<Operation> = vec![];
        let expression = self.elimination_of_implication(&mut operations)
            .de_morgans_laws(&mut operations)
            .absorption_law(&mut operations, options.ignore_case)
            .absorb_opposites(&mut operations, options.ignore_case)
            // .associative_law(&mut operations)
            .distributive_law(&mut operations)
            .double_negation_elimination(&mut operations);
        // .commutative_law(&mut operations);
        (expression, operations)
    }

    /// Eliminate the implication operator from the expression.
    /// This is done by replacing `a ➔ b` with `¬a ⋁ b`.
    fn elimination_of_implication(&self, operations: &mut Vec<Operation>) -> Self {
        match self {
            Expression::Not(expr) => {
                not(expr.elimination_of_implication(operations))
            }
            Expression::Binary { left, operator, right } => {
                let l_result = left.elimination_of_implication(operations);
                let r_result = right.elimination_of_implication(operations);
                let before = binary(l_result.clone(), *operator, r_result.clone());

                if let BinaryOperator::Implication = *operator {
                    let after = or(not(l_result.clone()), r_result.clone());
                    if let Some(operation) = Operation::new(&before, &after, Law::EliminationOfImplication) {
                        operations.push(operation);
                    }
                    after
                } else {
                    before
                }
            }
            atomic @ Expression::Atomic(_) => atomic.clone(),
        }
    }

    /// Eliminate double negations from the expression.
    /// This is done by replacing `¬¬a` with `a`.
    /// This function is recursive and will continue to eliminate double negations until none are left.
    fn double_negation_elimination(&self, operations: &mut Vec<Operation>) -> Self {
        match self {
            Expression::Not(expr) => {
                if let Expression::Not(after) = expr.deref() {
                    if let Some(operation) = Operation::new(self, after, Law::DoubleNegationElimination) {
                        operations.push(operation);
                    }
                    after.double_negation_elimination(operations)
                } else {
                    self.clone()
                }
            }
            Expression::Binary { left, operator, right } => {
                let left = left.double_negation_elimination(operations);
                let right = right.double_negation_elimination(operations);
                binary(left, *operator, right)
            }
            atomic @ Expression::Atomic(_) => atomic.clone(),
        }
    }

    fn de_morgans_laws(&self, operations: &mut Vec<Operation>) -> Self {
        match self {
            Expression::Not(expr) => {
                match expr.deref() {
                    Expression::Binary { left, operator: operator @ (BinaryOperator::And | BinaryOperator::Or), right } => {
                        let left = not(left.de_morgans_laws(operations));
                        let right = not(right.de_morgans_laws(operations));
                        let result = if let BinaryOperator::And = operator {
                            or(left, right)
                        } else {
                            and(left, right)
                        };
                        if let Some(operation) = Operation::new(self, &result, Law::DeMorgansLaws) {
                            operations.push(operation);
                        }
                        result.de_morgans_laws(operations)
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
        }
    }

    // TODO merge some branches?
    fn absorption_law(&self, operations: &mut Vec<Operation>, ignore_case: bool) -> Self {
        match self {
            Expression::Binary { left, operator: BinaryOperator::And | BinaryOperator::Or, right }
            if Expression::eq(left, right, ignore_case) => {
                if let Some(operation) = Operation::new(self, left, Law::AbsorptionLaw) {
                    operations.push(operation);
                }
                left.absorption_law(operations, ignore_case)
            }
            Expression::Binary { left, operator, right }
            if operator.is_and() && (right.is_in(left) && left.is_and() || left.is_in(right) && right.is_or()) => {
                if let Some(operation) = Operation::new(self, left, Law::AbsorptionLaw) {
                    operations.push(operation);
                }
                left.absorption_law(operations, ignore_case)
            }
            Expression::Binary { left, operator, right }
            if operator.is_and() && (right.is_in(left) && left.is_or() || left.is_in(right) && right.is_and()) => {
                if let Some(operation) = Operation::new(self, right, Law::AbsorptionLaw) {
                    operations.push(operation);
                }
                right.absorption_law(operations, ignore_case)
            }
            Expression::Binary { left, operator, right }
            if operator.is_or() && (right.is_in(left) && (left.is_and() || left.is_or()) || left.is_in(right) && right.is_or()) => {
                if let Some(operation) = Operation::new(self, right, Law::AbsorptionLaw) {
                    operations.push(operation);
                }
                right.absorption_law(operations, ignore_case)
            }
            Expression::Binary { left, operator: BinaryOperator::Or, right }
            if left.is_in(right) && right.is_and() => {
                if let Some(operation) = Operation::new(self, left, Law::AbsorptionLaw) {
                    operations.push(operation);
                }
                left.absorption_law(operations, ignore_case)
            }
            Expression::Binary { left, operator, right } => binary(
                left.absorption_law(operations, ignore_case),
                *operator,
                right.absorption_law(operations, ignore_case),
            ),
            Expression::Not(expr) => not(expr.absorption_law(operations, ignore_case)),
            atomic => atomic.clone(),
        }
    }

    fn absorb_opposites(&self, operations: &mut Vec<Operation>, ignore_case: bool) -> Self {
        match self {
            Expression::Binary { left, operator: BinaryOperator::And, right } => {
                // TODO Refactor duplicate code with absorption_law!
                absorption_law_opposites!(left, right, operations, BinaryOperator::And, BinaryOperator::Or, and, ignore_case)
            }
            Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                absorption_law_opposites!(left, right, operations, BinaryOperator::Or, BinaryOperator::And, or, ignore_case)
            }
            Expression::Binary { left, operator, right } => binary(
                left.absorb_opposites(operations, ignore_case),
                *operator,
                right.absorb_opposites(operations, ignore_case),
            ),
            Expression::Not(expr) => not(expr.absorb_opposites(operations, ignore_case)),
            atomic @ Expression::Atomic(_) => atomic.clone(),
        }
    }

    // A ⋀ (B ⋀ C) <=> (A ⋀ B) ⋀ C
    fn associative_law(&self, operations: &mut Vec<Operation>) -> Self {
        todo!("? | Associative law: (a ⋀ b) ⋀ c == a ⋀ (b ⋀ c) and (a ⋁ b) ⋁ c == a ⋁ (b ⋁ c)")
    }

    // A & (B | C) <=> A & B | A & C
    fn distributive_law(&self, operations: &mut Vec<Operation>) -> Self {
        match self {
            Expression::Binary { left, operator: BinaryOperator::And, right } => {
                distributive_law_atomic_vs_binary!(left, right, operations, BinaryOperator::Or, or, and)
            }
            Expression::Binary { left, operator: BinaryOperator::Or, right } => {
                distributive_law_atomic_vs_binary!(left, right, operations, BinaryOperator::And, and, or)
            }
            Expression::Binary { left, operator, right } => binary(
                left.distributive_law(operations),
                *operator,
                right.distributive_law(operations),
            ),
            Expression::Not(expr) => not(expr.distributive_law(operations)),
            atomic @ Expression::Atomic(_) => atomic.clone(),
        }
    }

    fn commutative_law(&self, operations: &mut Vec<Operation>) -> Self {
        todo!("? | Order of operands does not matter in AND and OR operations.")
    }
}

fn evaluate_equals_or_opposites<F: Fn(Expression, Expression) -> Expression>(
    this: &Expression,
    left: &Expression,
    right: &Expression,
    op_func: F, // TODO pass in BinaryOperator instead of function
    ignore_case: bool,
    operations: &mut Vec<Operation>,
) -> Option<Expression> {
    if this.eq(left, ignore_case) || this.eq(right, ignore_case) {
        return Some(this.absorption_law(operations, ignore_case));
    } else if left.is_atomic() && right.is_atomic() {
        if this.opposite_eq(left, ignore_case) {
            return Some(op_func(right.absorption_law(operations, ignore_case), this.absorption_law(operations, ignore_case)));
        } else if this.opposite_eq(right, ignore_case) {
            return Some(op_func(left.absorption_law(operations, ignore_case), this.absorption_law(operations, ignore_case)));
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
        let (expression, operations) = implies(atomic("a"), atomic("b")).simplify(Default::default());
        assert_eq!(expression, or(not(atomic("a")), atomic("b")));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::EliminationOfImplication);
    }

    #[test]
    fn test_simplify_a_and_a() {
        let (expression, operations) = and(atomic("a"), atomic("a")).simplify(Default::default());
        assert_eq!(expression, atomic("a"));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::AbsorptionLaw);
    }

    #[test]
    fn test_implication_and_de_morgans() {
        let expression = implies(and(not(atomic("a")), atomic("b")), atomic("c")).simplify(Default::default()).0;
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
        assert_eq!(operations[1].before, "a ➔ ¬b ⋁ c");
        assert_eq!(operations[1].after, "¬a ⋁ ¬b ⋁ c");
    }

    #[test]
    fn test_elimination_of_implication_none() {
        let mut operations = vec![];
        let expression = and(atomic("a"), atomic("b")).elimination_of_implication(&mut operations);
        assert_eq!(expression, and(atomic("a"), atomic("b")));
        assert_eq!(operations.len(), 0);
    }

    #[test]
    fn test_elimination_of_implication_nested_none() {
        let mut operations = vec![];
        let expression = or(atomic("a"), and(atomic("b"), atomic("c"))).elimination_of_implication(&mut operations);
        assert_eq!(expression, or(atomic("a"), and(atomic("b"), atomic("c"))));
        assert_eq!(operations.len(), 0);
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
        assert_eq!(operations[0].before, "¬¬¬¬¬a");
        assert_eq!(operations[0].after, "¬¬¬a");
        assert_eq!(operations[1].law, Law::DoubleNegationElimination);
        assert_eq!(operations[1].before, "¬¬¬a");
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
        assert_eq!(operations[0].before, "¬(a ⋁ b)");
        assert_eq!(operations[0].after, "¬a ⋀ ¬b");
    }

    #[test]
    fn test_de_morgans_laws_nested_or() {
        let mut operations = vec![];
        let expression = not(or(and(atomic("a"), atomic("b")), atomic("c"))).de_morgans_laws(&mut operations); // ¬(a ⋀ b ⋁ c)
        assert_eq!(expression, and(or(not(atomic("a")), not(atomic("b"))), not(atomic("c")))); // ¬(a ⋀ b) ⋀ ¬c == (¬a ⋁ ¬b) ⋀ ¬c
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::DeMorgansLaws);
        assert_eq!(operations[0].before, "¬(a ⋀ b ⋁ c)");
        assert_eq!(operations[0].after, "¬(a ⋀ b) ⋀ ¬c");
        assert_eq!(operations[1].law, Law::DeMorgansLaws);
        assert_eq!(operations[1].before, "¬(a ⋀ b)");
        assert_eq!(operations[1].after, "¬a ⋁ ¬b");
    }

    #[test]
    fn test_de_morgans_laws_nested_and() {
        let mut operations = vec![];
        let expression = not(and(or(atomic("a"), atomic("b")), atomic("c"))).de_morgans_laws(&mut operations); // ¬((a ⋁ b) ⋀ c)
        assert_eq!(expression, or(and(not(atomic("a")), not(atomic("b"))), not(atomic("c")))); // ¬(a ⋁ b) ⋀ ¬c == (¬a ⋀ ¬b) ⋁ ¬c
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::DeMorgansLaws);
        assert_eq!(operations[0].before, "¬((a ⋁ b) ⋀ c)");
        assert_eq!(operations[0].after, "¬(a ⋁ b) ⋁ ¬c");
        assert_eq!(operations[1].law, Law::DeMorgansLaws);
        assert_eq!(operations[1].before, "¬(a ⋁ b)");
        assert_eq!(operations[1].after, "¬a ⋀ ¬b");
    }

    #[test]
    fn test_de_morgans_laws_nested_and_or() {
        let mut operations = vec![];
        let expression = not(and(or(atomic("a"), atomic("b")), or(atomic("c"), atomic("d")))).de_morgans_laws(&mut operations); // ¬(a ⋁ b ⋀ c ⋁ d)
        assert_eq!(expression, or(and(not(atomic("a")), not(atomic("b"))), and(not(atomic("c")), not(atomic("d"))))); // ¬(a ⋁ b) ⋀ ¬(c ⋁ d) == (¬a ⋀ ¬b) ⋁ (¬c ⋀ ¬d)
        assert_eq!(operations.len(), 3);
    }

    // a & (a | b) <=> a
    #[test]
    fn test_absorption_law_and() {
        let mut operations = vec![];
        let expression = and(atomic("a"), or(atomic("a"), atomic("b"))).absorption_law(&mut operations, false);
        assert_eq!(expression, atomic("a"));
        assert_eq!(operations.len(), 1);
    }

    #[test]
    fn test_absorption_law_or() {
        let mut operations = vec![];
        let expression = or(atomic("a"), and(atomic("a"), atomic("b"))).absorption_law(&mut operations, false);
        assert_eq!(expression, atomic("a"));
        assert_eq!(operations.len(), 1);
    }

    // !A | !A | A | B <=> !A | A | B
    #[test]
    fn test_absorption_law_duplicate() {
        let mut operations = vec![];
        let expression = or(
            not(atomic("a")),
            or(
                not(atomic("a")),
                or(
                    atomic("a"),
                    atomic("b"),
                ),
            ),
        ).absorption_law(&mut operations, false);
        assert_eq!(expression, or(not(atomic("a")), or(atomic("a"), atomic("b"))));
        assert_eq!(operations.len(), 1);
    }


    // !A & B | A <=> B | A
    #[test]
    fn test_absorption_law_not() {
        let mut operations = vec![];
        let expression = or(and(not(atomic("a")), atomic("b")), atomic("a")).absorb_opposites(&mut operations, Default::default());
        assert_eq!(expression, or(atomic("b"), atomic("a")));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::AbsorptionLaw);
        assert_eq!(operations[0].before, "¬a ⋀ b ⋁ a");
        assert_eq!(operations[0].after, "b ⋁ a");
    }

    #[test]
    fn test_absorption_law_duplicate_not() {
        let mut operations = vec![];
        let expression = and(not(atomic("A")), not(atomic("A")));
        let simplified = expression.absorption_law(&mut operations, Default::default());
        assert_eq!(simplified, not(atomic("A")));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::AbsorptionLaw);
        assert_eq!(operations[0].before, "¬A ⋀ ¬A");
        assert_eq!(operations[0].after, "¬A");
    }

    // A & B | !A <=> B | !A
    #[test]
    fn test_absorption_law_not_reversed() {
        let mut operations = vec![];
        let expression = or(and(atomic("a"), atomic("b")), not(atomic("a"))).absorb_opposites(&mut operations, Default::default());
        assert_eq!(expression, or(atomic("b"), not(atomic("a"))));
        assert_eq!(operations.len(), 1);
    }

    // A | !A | B <=> A | !A
    #[test]
    fn test_absorption_law_not_duplicate() {
        let mut operations = vec![];
        let expression = or(atomic("a"), or(not(atomic("a")), atomic("b"))).absorb_opposites(&mut operations, Default::default());
        assert_eq!(expression, or(atomic("a"), not(atomic("a"))));
        assert_eq!(operations.len(), 1);
    }

    // !A & B | !A <=> !A
    #[test]
    fn test_absorption_law_double_not() {
        let mut operations = vec![];
        let expression = or(and(not(atomic("a")), atomic("b")), not(atomic("a"))).absorption_law(&mut operations, Default::default());
        assert_eq!(expression, not(atomic("a")));
        assert_eq!(operations.len(), 1);
    }

    #[test]
    fn test_absorption_law_duplicate_atomic() {
        let mut operations = vec![];
        let expression = and(atomic("A"), atomic("A"));
        let simplified = expression.absorption_law(&mut operations, Default::default());
        assert_eq!(simplified, atomic("A"));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::AbsorptionLaw);
        assert_eq!(operations[0].before, "A ⋀ A");
        assert_eq!(operations[0].after, "A");
    }

    #[test]
    fn test_absorption_law_double_duplicates() {
        let mut operations = vec![];
        let expression = and(or(atomic("A"), atomic("A")), or(atomic("B"), atomic("B")));
        let simplified = expression.absorption_law(&mut operations, Default::default());
        assert_eq!(simplified, and(atomic("A"), atomic("B")));
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::AbsorptionLaw);
        assert_eq!(operations[0].before, "A ⋁ A");
        assert_eq!(operations[0].after, "A");
        assert_eq!(operations[1].law, Law::AbsorptionLaw);
        assert_eq!(operations[1].before, "B ⋁ B");
        assert_eq!(operations[1].after, "B");
    }

    // (A | B) & !A <=> B & !A
    #[test]
    fn test_in_parenthesis() {
        let mut operations = vec![];
        let expression = and(or(atomic("a"), atomic("b")), not(atomic("a"))).absorb_opposites(&mut operations, Default::default());
        assert_eq!(expression, and(atomic("b"), not(atomic("a"))));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::AbsorptionLaw);
        assert_eq!(operations[0].before, "(a ⋁ b) ⋀ ¬a");
        assert_eq!(operations[0].after, "b ⋀ ¬a");
    }

    #[test]
    fn test_distributive_law_and() {
        let mut operations = vec![];
        let expression = and(atomic("a"), or(atomic("b"), atomic("c"))).distributive_law(&mut operations);
        assert_eq!(expression, or(and(atomic("a"), atomic("b")), and(atomic("a"), atomic("c"))));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DistributiveLaw);
        assert_eq!(operations[0].before, "a ⋀ (b ⋁ c)");
        assert_eq!(operations[0].after, "a ⋀ b ⋁ a ⋀ c");
    }

    #[test]
    fn test_distributive_law_or() {
        let mut operations = vec![];
        let expression = or(atomic("a"), and(atomic("b"), atomic("c"))).distributive_law(&mut operations);
        assert_eq!(expression, and(or(atomic("a"), atomic("b")), or(atomic("a"), atomic("c"))));
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].law, Law::DistributiveLaw);
        assert_eq!(operations[0].before, "a ⋁ b ⋀ c");
        assert_eq!(operations[0].after, "(a ⋁ b) ⋀ (a ⋁ c)");
    }

    #[test]
    fn test_distributive_law_nested_not() {
        let mut operations = vec![];
        let expression = and(atomic("a"), not(or(atomic("b"), atomic("c")))).distributive_law(&mut operations);
        assert_eq!(expression, and(atomic("a"), not(or(atomic("b"), atomic("c")))));
        assert_eq!(operations.len(), 0);
    }

    #[test]
    fn test_distributive_law_duplicates() {
        let mut operations = vec![];
        let expression = and(
            and(atomic("A"), or(atomic("B"), atomic("C"))),
            and(atomic("A"), or(atomic("B"), atomic("C"))),
        ).distributive_law(&mut operations);
        assert_eq!(expression, and(
            or(and(atomic("A"), atomic("B")), and(atomic("A"), atomic("C"))),
            or(and(atomic("A"), atomic("B")), and(atomic("A"), atomic("C"))),
        ));
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].law, Law::DistributiveLaw);
        assert_eq!(operations[0].before, "A ⋀ (B ⋁ C)");
        assert_eq!(operations[0].after, "A ⋀ B ⋁ A ⋀ C");
        assert_eq!(operations[1].law, Law::DistributiveLaw);
        assert_eq!(operations[1].before, "A ⋀ (B ⋁ C)");
        assert_eq!(operations[1].after, "A ⋀ B ⋁ A ⋀ C");
    }
}
