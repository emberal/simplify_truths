use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use derive::IntoResponse;

use crate::expressions::expression::Expression;
use crate::expressions::simplify::Law;
use crate::expressions::truth_table::TruthTable;

#[derive(Serialize)]
struct BaseResponse<T: Serialize> {
    version: String,
    #[serde(flatten)]
    result: T,
}

impl<T: Serialize> BaseResponse<T> {
    fn create(result: T) -> Response {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            result,
        }.into_response()
    }
}

impl<T: Serialize> IntoResponse for BaseResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Operation {
    pub before: String,
    pub after: String,
    pub law: Law,
}

impl Operation {
    pub fn new(before: &Expression, after: &Expression, law: Law) -> Option<Self> {
        if *before != *after {
            Some(Self { before: before.to_string(), after: after.to_string(), law })
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
    pub const fn invalid() -> Self {
        Self { is_valid: false }
    }
}

#[derive(Serialize, IntoResponse)]
#[serde(rename_all = "camelCase")]
pub struct TruthTableResponse {
    pub truth_table: TruthTable,
}