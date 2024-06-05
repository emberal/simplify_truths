use axum::{Router, routing::get};
use axum::extract::{Path, Query};
use serde::Deserialize;

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
    #[serde(default)]
    case_sensitive: bool,
}

// TODO
async fn simplify(Path(path): Path<String>, query: Query<QueryOptions>, accept_language: Option<AcceptLanguage>) -> String {
    format!("Path: {}, Query: {:?}, Accept-language header: {:?}", path, query, accept_language)
}

async fn simplify_and_table() {
    unimplemented!("Not yet implemented")
}
