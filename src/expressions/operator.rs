use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BinaryOperator {
    Implication,
    Or,
    And,
}

impl BinaryOperator {
    pub fn eval(&self, left: bool, right: bool) -> bool {
        match self {
            BinaryOperator::And => left && right,
            BinaryOperator::Or => left || right,
            BinaryOperator::Implication => !left || right,
        }
    }
}
