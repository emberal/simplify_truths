use serde::{Deserialize, Serialize};

use crate::expressions::expression::Expression;
use crate::matrix;
use crate::utils::array::{alternating_array, Distinct};

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
        let helper = Self::helper_matrix(count);
        for row in &helper {
            let truths = Self::generate_truth_table(row, expression);
        }
        todo!()
    }

    fn helper_matrix(number_of_atomics: usize) -> TruthMatrix {
        let len = 2usize.pow(number_of_atomics as u32);
        let mut change_index = len / 2;
        let mut rows: Vec<Vec<bool>> = matrix![false; 0 => number_of_atomics];
        for row in &mut rows {
            *row = alternating_array(len, change_index);
            change_index /= 2;
        }
        rows
    }

    // TODO store the expressions along with their values in a list tree structure
    // For each node. Their left child is index * 2 + 1 and right child is index * 2 + 2
    // Ex: 0 -> (1, 2), 1 -> (3, 4), 2 -> (5, 6)
    fn generate_truth_table<'a>(truth_row: &[bool], expression: &'a Expression) -> Vec<Option<(&'a Expression, bool)>> {
        match expression {
            not @ Expression::Not(expr) => {
                [
                    vec![Some((not, Self::resolve_expression(not, truth_row)))],
                    Self::generate_truth_table(truth_row, expr),
                    vec![None]
                ].concat()
            }
            binary @ Expression::Binary { left, right, .. } => {
                [
                    vec![Some((binary, Self::resolve_expression(binary, truth_row)))],
                    Self::generate_truth_table(truth_row, left),
                    Self::generate_truth_table(truth_row, right)
                ].concat()
            }
            atomic @ Expression::Atomic(_) => {
                vec![Some((expression, Self::resolve_expression(atomic, truth_row)))]
            }
        }
    }

    fn resolve_expression(expression: &Expression, helper: &[bool]) -> bool {
        todo!("Resolve the expression with the given row of booleans")
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    use super::*;

    #[test]
    fn test_helper_matrix_3() {
        let helper = TruthTable::helper_matrix(3);
        assert_eq!(helper, matrix![
            true, true, true, true, false, false, false, false;
            true, true, false, false, true, true, false, false;
            true, false, true, false, true, false, true, false
        ]);
    }

    #[test]
    fn test_helper_matrix_1() {
        let helper = TruthTable::helper_matrix(1);
        assert_eq!(helper, matrix![true, false]);
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
