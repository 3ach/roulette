mod app;
mod db;
mod models;

use dotenvy;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let router = app::app().await;
    axum::serve(listener, router).await.unwrap();
}
