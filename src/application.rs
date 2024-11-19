use crate::router::script;
use crate::utils::config::arg::ServiceConfig;
use crate::utils::config::script_mapper::ScriptMapper;
use std::net::SocketAddr;

pub async fn app() -> anyhow::Result<()> {
    ServiceConfig::init().await;
    ScriptMapper::init().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr).serve(script::route().into_make_service()).await?;
    Ok(())
}
