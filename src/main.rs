use crate::utils::config::arg::ServiceConfig;
use crate::utils::config::script_mapper::ScriptMapper;

mod application;
mod middleware;
mod router;
mod service;
mod utils;

#[tokio::main]
async fn main() {
    application::app().await.expect("App Exited With Error");
}
