use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::expressions::expression::Expression;
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

#[derive(Serialize)]
enum Law {
    // TODO
}

#[derive(Serialize)]
pub struct OrderOfOperation {
    before: String,
    after: String,
    law: Law, // TODO
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimplifyResponse {
    pub before: String,
    pub after: String,
    pub order_of_operations: Vec<OrderOfOperation>,
    pub expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truth_table: Option<TruthTable>,
}

impl IntoResponse for SimplifyResponse {
    fn into_response(self) -> Response {
        BaseResponse::create(self)
    }
}