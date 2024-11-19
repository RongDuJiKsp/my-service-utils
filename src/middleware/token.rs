use crate::utils::config::arg::ServiceConfig;
use axum::extract::Query;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use std::collections::HashMap;

const TOKEN_PARAM: &str = "token";
pub async fn check_token_param<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let cfg_token = match &ServiceConfig::get().await.handle_token {
        Some(e) => e,
        None => return Ok(next.run(req).await),
    };
    let Query(q) = match Query::<HashMap<String, String>>::try_from_uri(req.uri()) {
        Ok(e) => e,
        Err(_) => return no_token_err(),
    };
    let token = match q.get(TOKEN_PARAM) {
        None => return no_token_err(),
        Some(e) => e,
    };
    if token != cfg_token {
        return no_token_err();
    }
    Ok(next.run(req).await)
}
fn no_token_err() -> Result<Response, StatusCode> {
    Ok((
        StatusCode::FORBIDDEN,
        "Custom Response: You cannot access this route of Not Token.",
    )
        .into_response())
}
