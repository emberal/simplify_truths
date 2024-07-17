use axum::response::{IntoResponse, Response};
use lib::into_response_derive::IntoResponse;
use serde::Serialize;

use crate::expressions::expression::Expression;
use crate::expressions::simplify::Law;
use crate::expressions::truth_table::TruthTable;

#[derive(Debug, PartialEq, Serialize)]
pub struct Operation {
    pub before: String,
    pub after: String,
    pub law: Law,
}

impl Operation {
    pub fn new(before: &Expression, after: &Expression, law: Law) -> Option<Self> {
        if before != after {
            Some(Self {
                before: before.to_string(),
                after: after.to_string(),
                law,
            })
        } else {
            None
        }
    }
}

#[derive(Serialize, IntoResponse)]
#[serde(rename_all = "camelCase")]
pub struct SimplifyResponse {
    pub before: String,
    pub after: String,
    pub operations: Vec<Operation>,
    pub expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truth_table: Option<TruthTable>,
}

#[derive(Serialize, IntoResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IsValidResponse {
    pub is_valid: bool,
}

impl IsValidResponse {
    pub const fn valid() -> Self {
        Self { is_valid: true }
    }
}

#[derive(Serialize, IntoResponse)]
#[serde(rename_all = "camelCase")]
pub struct TruthTableResponse {
    pub truth_table: TruthTable,
}
