use crate::middleware::token;
use axum::extract::{Path, Query};
use axum::routing::{get, Router};
use std::collections::HashMap;
use axum::middleware;

pub fn route() -> Router {
    Router::new()
        .route("/script/:script", get(run_script))
        .route_layer(middleware::from_fn(token::check_token_param))
}

async fn run_script(
    Query(param): Query<HashMap<String, String>>,
    Path(script): Path<String>,
) -> String {

}
