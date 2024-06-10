use std::slice::Iter;

use serde::{Deserialize, Serialize};

use crate::expressions::expression::Expression;
use crate::expressions::operator::BinaryOperator;
use crate::utils::array::Distinct;

type TruthMatrix = Vec<Vec<bool>>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TruthTable {
    header: Vec<String>,
    truth_matrix: TruthMatrix,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Hide {
    #[default]
    None,
    True,
    False,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Sort {
    #[default]
    Default,
    TrueFirst,
    FalseFirst,
}

#[derive(Debug, Default, Deserialize)]
pub struct TruthTableOptions {
    pub sort: Sort,
    pub hide: Hide,
}

impl TruthTable {
    pub fn new(expression: &Expression, options: TruthTableOptions) -> Self {
        let header = Self::extract_header(expression);
        let truth_matrix = Self::generate_truth_matrix(expression);
        Self { header, truth_matrix }
    }

    /// Extracts the header for the truth table from the expression
    /// Duplicate values are removed.
    /// - Arguments
    ///     - `expression` - The expression to extract the header from
    /// - Returns
    ///     - A vector of strings representing the header
    /// # Example
    /// ```
    /// let expression = TruthTable::extract_header(&atomic!("A"));
    /// let complex_expression = TruthTable::extract_header(&implies!(and!(atomic!("A"), atomic!("B")), or!(atomic!("C"), atomic!("D"))));
    /// assert_eq!(expression, vec!["A"]);
    /// assert_eq!(complex_expression, vec!["A", "B", "A ⋀ B", "C", "D", "(C ⋁ D)", "A ⋀ B ➔ (C ⋁ D)"]);
    /// ```
    fn extract_header(expression: &Expression) -> Vec<String> {
        match expression {
            not @ Expression::Not(expr) => {
                let mut header = Self::extract_header(expr);
                header.push(not.to_string());
                header.distinct();
                header
            }
            binary @ Expression::Binary { left, right, .. } => {
                let mut header = Self::extract_header(left);
                header.extend(Self::extract_header(right));
                header.push(binary.to_string());
                header.distinct();
                header
            }
            Expression::Atomic(value) => vec![value.clone()],
        }
    }

    fn generate_truth_matrix(expression: &Expression) -> TruthMatrix {
        let count = expression.count_distinct();
        if count == 0 {
            return vec![];
        }
        Self::truth_combinations(count)
            .iter().map(|combo| {
            Self::resolve_expression(expression, &mut combo.iter())
        }).collect()
    }

    fn truth_combinations(count: usize) -> TruthMatrix {
        (0..2usize.pow(count as u32))
            .map(|i| (0..count).rev()
                // Just trust me bro
                .map(|j| (i >> j) & 1 == 0).collect()
            ).collect()
    }

    fn resolve_expression(expression: &Expression, booleans: &mut Iter<bool>) -> Vec<bool> {
        match expression {
            Expression::Not(expr) => {
                Self::resolve_expression(expr, booleans)
                    .iter().map(|value| !value).collect()
            }
            Expression::Binary { left, right, .. } => {
                let left_values = Self::resolve_expression(left, booleans);
                let right_values = Self::resolve_expression(right, booleans);
                left_values.iter()
                    .zip(right_values.iter())
                    .flat_map(|(left_value, right_value)| {
                        [*left_value, *right_value, match expression {
                            Expression::Binary { operator: BinaryOperator::And, .. } => *left_value && *right_value,
                            Expression::Binary { operator: BinaryOperator::Or, .. } => *left_value || *right_value,
                            Expression::Binary { operator: BinaryOperator::Implication, .. } => !*left_value || *right_value,
                            _ => false,
                        }]
                    }).collect()
            }
            Expression::Atomic(_) => {
                if let Some(value) = booleans.next() {
                    vec![*value]
                } else {
                    vec![]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;
    use super::*;

    #[test]
    fn test_new_truth_table() {
        let expression = and!(atomic!("A"), atomic!("B"));
        let truth_table = TruthTable::new(&expression, Default::default());
        assert_eq!(truth_table.header, vec!["A", "B", "A ⋀ B"]);
        assert_eq!(truth_table.truth_matrix, matrix![
            true, true, true;
            true, false, false;
            false, true, false;
            false, false, false
        ]);
    }

    #[test]
    fn test_truth_combinations() {
        let combinations = TruthTable::truth_combinations(3);
        assert_eq!(combinations, matrix![
            true, true, true;
            true, true, false;
            true, false, true;
            true, false, false;
            false, true, true;
            false, true, false;
            false, false, true;
            false, false, false
        ]);
    }

    #[test]
    fn test_resolve_expression_and_all_true() {
        let expression = and!(atomic!("A"), atomic!("B"));
        let booleans = [true, true];
        let values = TruthTable::resolve_expression(&expression, &mut booleans.iter());
        assert_eq!(values, vec![true, true, true]);
    }

    #[test]
    fn test_resolve_expression_and_1_true_1_false() {
        let expression = and!(atomic!("A"), atomic!("B"));
        let booleans = [true, false];
        let values = TruthTable::resolve_expression(&expression, &mut booleans.iter());
        assert_eq!(values, vec![true, false, false]);
    }

    #[test]
    fn test_resolve_expression_or_1_true_1_false() {
        let expression = or!(atomic!("A"), atomic!("B"));
        let booleans = [true, false];
        let values = TruthTable::resolve_expression(&expression, &mut booleans.iter());
        assert_eq!(values, vec![true, false, true]);
    }

    #[test]
    fn test_atomic_expression() {
        let expression = atomic!("A");
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A"]);
    }

    #[test]
    fn test_not_expression() {
        let expression = not!(atomic!("A"));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "¬A"]);
    }

    #[test]
    fn test_binary_and_expression() {
        let expression = and!(atomic!("A"), atomic!("B"));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "B", "A ⋀ B"]);
    }

    #[test]
    fn test_binary_or_expression() {
        let expression = or!(atomic!("A"), atomic!("B"));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "B", "(A ⋁ B)"]);
    }

    #[test]
    fn test_binary_implies_expression() {
        let expression = implies!(atomic!("A"), atomic!("B"));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "B", "A ➔ B"]);
    }

    #[test]
    fn test_complex_expression() {
        let expression = implies!(and!(atomic!("A"), atomic!("B")), or!(atomic!("C"), atomic!("D")));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "B", "A ⋀ B", "C", "D", "(C ⋁ D)", "A ⋀ B ➔ (C ⋁ D)"]);
    }

    #[test]
    fn test_equal_expressions_should_not_duplicate() {
        let expression = and!(atomic!("A"), and!(atomic!("A"), and!(atomic!("A"), atomic!("A"))));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "A ⋀ A", "A ⋀ A ⋀ A", "A ⋀ A ⋀ A ⋀ A"]);
    }

    #[test]
    fn test_somewhat_equal() {
        let expression = and!(atomic!("A"), and!(or!(not!(atomic!("A")), atomic!("B")), atomic!("A")));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "¬A", "B", "(¬A ⋁ B)", "(¬A ⋁ B) ⋀ A", "A ⋀ (¬A ⋁ B) ⋀ A"]);
    }
}
