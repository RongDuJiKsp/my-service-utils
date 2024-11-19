use crate::router::script;
use crate::utils::config::arg::ServiceConfig;
use crate::utils::config::script_mapper::ScriptMapper;
use axum::Router;

pub async fn app() -> anyhow::Result<()> {
    ServiceConfig::init().await;
    ScriptMapper::init().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
    let router = Router::new().merge(script::route());
    axum::serve(listener, router.into_make_service())
        .await?;
    Ok(())
}
