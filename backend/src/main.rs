use tokio;

mod router;
mod model;

use router::construct_router;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, construct_router()).await.unwrap();
}
