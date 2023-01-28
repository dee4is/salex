use std::net::SocketAddr;

mod routes;

use salex_core::extractors::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let app = routes::router().await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
