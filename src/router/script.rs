use std::collections::HashMap;
use axum::extract::{Path, Query};
use axum::routing::{get, Router};



pub fn route() -> Router {
    Router::new().route("/script/:script", get(run_script))
}

async fn run_script(Query(param): Query<HashMap<String, String>>, Path(script): Path<String>) -> String {

}