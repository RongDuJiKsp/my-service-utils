use std::net::SocketAddr;
use crate::router::script_call;

pub async fn app() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(script_call::route())
        .await?;
    Ok(())
}