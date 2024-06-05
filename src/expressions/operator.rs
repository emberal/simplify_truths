#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BinaryOperator {
    Implication,
    Or,
    And,
}

impl From<BinaryOperator> for &str {
    fn from(op: BinaryOperator) -> Self {
        match op {
            BinaryOperator::Implication => "=>",
            BinaryOperator::Or => "|",
            BinaryOperator::And => "&",
        }
    }
}

