use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BinaryOperator {
    Implication,
    Or,
    And,
}
