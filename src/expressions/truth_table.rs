use std::cmp::Ordering;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::expressions::expression::Expression;
use crate::map;
use crate::utils::array::Distinct;

type TruthMatrix = Vec<Vec<bool>>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TruthTable {
    header: Vec<String>,
    truth_matrix: TruthMatrix,
}

#[derive(Debug, Default, Copy, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Hide {
    #[default]
    None,
    True,
    False,
}

#[derive(Debug, Default, Copy, Clone, Deserialize)]
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
    // TODO hide option
    pub fn new(expression: &Expression, options: TruthTableOptions) -> Self {
        let header = Self::extract_header(expression);
        let mut truth_matrix = Self::generate_truth_matrix(expression, &header);
        if !matches!(options.sort, Sort::Default) {
            Self::sort_matrix(&mut truth_matrix, options.sort);
        }
        Self { header, truth_matrix }
    }

    fn sort_matrix(truth_matrix: &mut TruthMatrix, sort: Sort) {
        truth_matrix.sort_by(|row_a, row_b| match sort {
            Sort::TrueFirst => row_b.last().cmp(&row_a.last()),
            Sort::FalseFirst => row_a.last().cmp(&row_b.last()),
            Sort::Default => Ordering::Equal,
        })
    }

    /// Extracts the header for the truth table from the expression
    /// Duplicate values are removed.
    /// - Arguments
    ///     - `expression` - The expression to extract the header from
    /// - Returns
    ///     - A vector of strings representing the header
    /// # Example
    /// ```
    /// let expression = TruthTable::extract_header(&atomic("A"));
    /// let complex_expression = TruthTable::extract_header(&implies(and(atomic("A"), atomic("B")), or(atomic("C"), atomic("D"))));
    /// assert_eq!(expression, vec!["A"]);
    /// assert_eq!(complex_expression, vec!["A", "B", "A ⋀ B", "C", "D", "(C ⋁ D)", "A ⋀ B ➔ (C ⋁ D)"]);
    /// ```
    fn extract_header(expression: &Expression) -> Vec<String> {
        match expression {
            Expression::Not(expr) => {
                let mut header = Self::extract_header(expr);
                header.push(expression.to_string());
                header.distinct();
                header
            }
            Expression::Binary { left, right, .. } => {
                let mut header = Self::extract_header(left);
                header.extend(Self::extract_header(right));
                header.push(expression.to_string());
                header.distinct();
                header
            }
            Expression::Atomic(value) => vec![value.clone()],
        }
    }

    fn generate_truth_matrix(expression: &Expression, header: &[String]) -> TruthMatrix {
        let mut atomics = expression.get_atomic_values()
            .into_iter().collect::<Vec<String>>();
        if atomics.is_empty() {
            return vec![];
        }
        atomics.sort();
        Self::truth_combinations(atomics.len()).iter()
            .map(|combo| {
                Self::resolve_expression(expression, &atomics.iter()
                    .enumerate()
                    .map(|(index, value)| (value.clone(), combo[index]))
                    .collect(), header)
            }).collect()
    }

    fn truth_combinations(count: usize) -> TruthMatrix {
        (0..2usize.pow(count as u32))
            .map(|i| (0..count).rev()
                // Just trust me bro
                .map(|j| (i >> j) & 1 == 0).collect()
            ).collect()
    }

    fn resolve_expression(expression: &Expression, booleans: &HashMap<String, bool>, header: &[String]) -> Vec<bool> {
        let expression_map = Self::_resolve_expression(expression, booleans);
        let string_map = expression_map.iter()
            .map(|(key, value)| (key.to_string(), *value))
            .collect::<HashMap<String, bool>>();

        header.iter()
            .map(|s_expr| string_map.get(s_expr).copied().expect("Expression not found in map"))
            .collect()
    }

    fn _resolve_expression<'a>(expression: &'a Expression, booleans: &HashMap<String, bool>) -> HashMap<&'a Expression, bool> {
        match expression {
            Expression::Not(expr) => {
                let mut map = Self::_resolve_expression(expr, booleans);
                if let Some(value) = map.get(expr.as_ref()) {
                    map.insert(expression, !value);
                }
                map
            }
            Expression::Binary { left, right, operator } => {
                let left_map = Self::_resolve_expression(left, booleans);
                let right_map = Self::_resolve_expression(right, booleans);
                let mut map = left_map;
                map.extend(right_map);
                if let (Some(left_value), Some(right_value)) = (map.get(left.as_ref()), map.get(right.as_ref())) {
                    map.insert(expression, operator.eval(*left_value, *right_value));
                }
                map
            }
            Expression::Atomic(value) => {
                if let Some(value) = booleans.get(value) {
                    map!(expression => *value)
                } else {
                    unreachable!("Atomic value not found in booleans")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::helpers::{and, atomic, implies, not, or};
    use crate::matrix;

    use super::*;

    #[test]
    fn test_new_truth_table() {
        let expression = and(atomic("A"), atomic("B"));
        let truth_table = TruthTable::new(&expression, Default::default());
        assert_eq!(truth_table.header, vec!["A", "B", "A ⋀ B"]);
        assert_ne!(truth_table.truth_matrix, matrix![
            true, true, true;
            false, true, false;
            true, false, false;
            false, false, false
        ]);
        assert_eq!(truth_table.truth_matrix, matrix![
            true, true, true;
            true, false, false;
            false, true, false;
            false, false, false
        ]);
    }

    #[test]
    fn test_new_truth_table_a_and_b_or_c() {
        let expression = and(or(atomic("A"), atomic("C")), or(atomic("B"), atomic("C")));
        let truth_table = TruthTable::new(&expression, Default::default());
        let atomics = 3;

        assert_eq!(truth_table.header, vec!["A", "C", "(A ⋁ C)", "B", "(B ⋁ C)", "(A ⋁ C) ⋀ (B ⋁ C)"]);
        assert_eq!(truth_table.truth_matrix.len(), 2usize.pow(atomics as u32));
        assert_eq!(truth_table.truth_matrix[0].len(), 6);
        assert_eq!(truth_table.truth_matrix[0], vec![true, true, true, true, true, true]);
        assert_eq!(truth_table.truth_matrix[1], vec![true, false, true, true, true, true]);
        assert_eq!(truth_table.truth_matrix[2], vec![true, true, true, false, true, true]);
        assert_eq!(truth_table.truth_matrix[3], vec![true, false, true, false, false, false]);
        assert_eq!(truth_table.truth_matrix[4], vec![false, true, true, true, true, true]);
        assert_eq!(truth_table.truth_matrix[5], vec![false, false, false, true, true, false]);
        assert_eq!(truth_table.truth_matrix[6], vec![false, true, true, false, true, true]);
        assert_eq!(truth_table.truth_matrix[7], vec![false, false, false, false, false, false]);
    }

    #[test]
    fn test_sort_matrix_true_first() {
        let mut matrix = matrix![
            true, true, true;
            true, false, false;
            false, true, true;
            false, false, false
        ];
        TruthTable::sort_matrix(&mut matrix, Sort::TrueFirst);
        assert_eq!(matrix, matrix![
            true, true, true;
            false, true, true;
            true, false, false;
            false, false, false
        ]);
    }

    #[test]
    fn test_sort_matrix_true_first_all_false_should_not_change() {
        let mut matrix = matrix![
            false, true, false;
            false, true, false;
            true, false, false;
            true, false, false
        ];
        TruthTable::sort_matrix(&mut matrix, Sort::TrueFirst);
        assert_eq!(matrix, matrix![
            false, true, false;
            false, true, false;
            true, false, false;
            true, false, false
        ]);
    }

    #[test]
    fn test_sort_matrix_default_should_not_change() {
        let mut matrix = matrix![
            true, true, true;
            true, false, false;
            false, true, true;
            false, false, false
        ];
        TruthTable::sort_matrix(&mut matrix, Sort::Default);
        assert_eq!(matrix, matrix![
            true, true, true;
            true, false, false;
            false, true, true;
            false, false, false
        ]);
    }

    #[test]
    fn test_sort_matrix_false_first() {
        let mut matrix = matrix![
            true, true, true;
            true, false, false;
            false, true, true;
            false, false, false
        ];
        TruthTable::sort_matrix(&mut matrix, Sort::FalseFirst);
        assert_eq!(matrix, matrix![
            true, false, false;
            false, false, false;
            true, true, true;
            false, true, true
        ]);
    }

    #[test]
    fn test_truth_combinations_2() {
        let combinations = TruthTable::truth_combinations(2);
        assert_eq!(combinations, matrix![
            true, true;
            true, false;
            false, true;
            false, false
        ]);
    }

    #[test]
    fn test_truth_combinations_3() {
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
        let expression = and(atomic("A"), atomic("B"));
        let booleans = map!["A".into() => true, "B".into() => true];
        let header = vec!["A".into(), "B".into(), "A ⋀ B".into()];
        let values = TruthTable::resolve_expression(&expression, &booleans, &header);
        assert_eq!(values, vec![true, true, true]);
    }

    #[test]
    fn test_resolve_expression_and_1_true_1_false() {
        let expression = and(atomic("A"), atomic("B"));
        let booleans = map!["A".into() => true, "B".into() => false];
        let header = vec!["A".into(), "B".into(), "A ⋀ B".into()];
        let values = TruthTable::resolve_expression(&expression, &booleans, &header);
        assert_eq!(values, vec![true, false, false]);
    }

    #[test]
    fn test_resolve_expression_or_1_true_1_false() {
        let expression = or(atomic("A"), atomic("B"));
        let booleans = map!["A".into() => true, "B".into() => false];
        let header = vec!["A".into(), "B".into(), "(A ⋁ B)".into()];
        let values = TruthTable::resolve_expression(&expression, &booleans, &header);
        assert_eq!(values, vec![true, false, true]);
    }

    #[test]
    fn test_resolve_expression_duplicate_atomic() {
        let expression = and(atomic("A"), atomic("A"));
        let booleans = map!["A".into() => true];
        let header = vec!["A".into(), "A ⋀ A".into()];
        let values = TruthTable::resolve_expression(&expression, &booleans, &header);
        assert_eq!(values, vec![true, true]);
    }

    #[test]
    fn test_resolve_expression_even_more_duplicates() {
        let expression = and(atomic("A"), and(atomic("A"), and(atomic("A"), atomic("A"))));
        let booleans = HashMap::from([("A".into(), true)]);
        let header = vec!["A".into(), "A ⋀ A".into(), "A ⋀ A ⋀ A".into(), "A ⋀ A ⋀ A ⋀ A".into()];
        let values = TruthTable::resolve_expression(&expression, &booleans, &header);
        assert_eq!(values, vec![true, true, true, true]);
    }

    #[test]
    fn _test_resolve_expression_even_more_duplicates() {
        let expression = and(atomic("A"), and(atomic("A"), and(atomic("A"), atomic("A"))));
        let booleans = HashMap::from([("A".into(), true)]);
        let values = TruthTable::_resolve_expression(&expression, &booleans);
        assert_eq!(values, HashMap::from([
            (&atomic("A"), true),
            (&and(atomic("A"), atomic("A")), true),
            (&and(atomic("A"), and(atomic("A"), atomic("A"))), true),
            (&and(atomic("A"), and(atomic("A"), and(atomic("A"), atomic("A")))), true),
        ]));
    }


    #[test]
    fn test_atomic_expression() {
        let expression = atomic("A");
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A"]);
    }

    #[test]
    fn test_not_expression() {
        let expression = not(atomic("A"));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "¬A"]);
    }

    #[test]
    fn test_binary_and_expression() {
        let expression = and(atomic("A"), atomic("B"));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "B", "A ⋀ B"]);
    }

    #[test]
    fn test_binary_or_expression() {
        let expression = or(atomic("A"), atomic("B"));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "B", "(A ⋁ B)"]);
    }

    #[test]
    fn test_binary_implies_expression() {
        let expression = implies(atomic("A"), atomic("B"));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "B", "A ➔ B"]);
    }

    #[test]
    fn test_complex_expression() {
        let expression = implies(and(atomic("A"), atomic("B")), or(atomic("C"), atomic("D")));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "B", "A ⋀ B", "C", "D", "(C ⋁ D)", "A ⋀ B ➔ (C ⋁ D)"]);
    }

    #[test]
    fn test_equal_expressions_should_not_duplicate() {
        let expression = and(atomic("A"), and(atomic("A"), and(atomic("A"), atomic("A"))));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "A ⋀ A", "A ⋀ A ⋀ A", "A ⋀ A ⋀ A ⋀ A"]);
    }

    #[test]
    fn test_somewhat_equal() {
        let expression = and(atomic("A"), and(or(not(atomic("A")), atomic("B")), atomic("A")));
        let header = TruthTable::extract_header(&expression);
        assert_eq!(header, vec!["A", "¬A", "B", "(¬A ⋁ B)", "(¬A ⋁ B) ⋀ A", "A ⋀ (¬A ⋁ B) ⋀ A"]);
    }
}
