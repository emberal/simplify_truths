use serde::{Deserialize, Serialize};

use crate::expressions::expression::Expression;
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
        todo!()
    }

    fn helper_matrix(number_of_atomics: usize) -> TruthMatrix {
        todo!("Create a matrix with 2^number_of_atomics rows and number_of_atomics columns")
    }

    fn resolve_expression(expression: &Expression, row: &[bool]) -> bool {
        todo!("Resolve the expression with the given row of booleans")
    }

    fn find_expression(expression: Expression, expressions: &[Expression]) -> Option<usize> {
        todo!("Find the expression in the truth table and return index")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
