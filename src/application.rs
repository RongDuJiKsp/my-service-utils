use crate::router::script;
use crate::utils::config::arg::ServiceConfig;
use crate::utils::config::script_mapper::ScriptMapper;
use axum::Router;

pub async fn app() -> anyhow::Result<()> {
    ServiceConfig::init().await;
    ScriptMapper::init().await;
    let addr = ServiceConfig::get().await.addr.clone();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Listening On {}", addr);
    let router = Router::new().merge(script::route());
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
