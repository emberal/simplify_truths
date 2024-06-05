use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{HeaderValue, StatusCode};
use axum::http::header::ACCEPT_LANGUAGE;
use axum::http::request::Parts;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub enum Language {
    #[default]
    #[serde(rename = "en")]
    En,
    #[serde(rename = "nb")]
    Nb,
}

#[derive(Debug)]
pub(crate) struct AcceptLanguage(HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for AcceptLanguage
    where
        S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(accept_language) = parts.headers.get(ACCEPT_LANGUAGE) {
            Ok(AcceptLanguage(accept_language.clone()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`Accept-language` header is missing"))
        }
    }
}
