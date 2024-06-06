use axum::{Json, Router, routing::get};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::expressions::expression::Expression;
use crate::expressions::simplify::Simplify;
use crate::language::{AcceptLanguage, Language};

pub fn router() -> Router<()> {
    Router::new()
        .nest("/simplify",
              Router::new()
                  .route("/:exp", get(simplify))
                  .route("/table/:exp", get(simplify_and_table)),
        )
}

const fn default_true() -> bool {
    true
}

#[derive(Deserialize, Debug)]
struct QueryOptions {
    #[serde(default)]
    lang: Language,
    #[serde(default = "default_true")]
    simplify: bool,
    #[serde(default = "default_true")]
    case_sensitive: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SimplifyResponse {
    before: String,
    after: String,
    order_of_operations: Vec<String>,
    expression: Expression,
}

// TODO
async fn simplify(Path(path): Path<String>, query: Query<QueryOptions>, accept_language: Option<AcceptLanguage>) -> Response {
    if let Ok(expression) = Expression::try_from(path.as_str()) {
        let simplified = expression.simplify();
        Json(SimplifyResponse {
            before: expression.to_string(),
            after: simplified.to_string(),
            order_of_operations: vec![], // TODO
            expression: simplified,
        }).into_response()
    } else {
        (StatusCode::BAD_REQUEST, "Invalid expression").into_response()
    }
}

async fn simplify_and_table() {
    unimplemented!("Not yet implemented")
}
