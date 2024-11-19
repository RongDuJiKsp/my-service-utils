use std::net::SocketAddr;
use crate::router::script;

pub async fn app() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(script::route())
        .await?;
    Ok(())
}