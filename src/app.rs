use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::sse::{Event, KeepAlive, Sse},
    routing::post,
    Router,
};
use futures::{Stream, StreamExt};
use sqlx::MySqlPool;
use std::convert::Infallible;
use std::env::var;

use crate::models::model::select;
use crate::db;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

pub async fn app() -> Router {
    let pool = db::pool().await.unwrap(); 
    let state = AppState { pool };

    Router::new().route("/prompt", post(prompt)).with_state(state)
}

async fn prompt(
    State(state): State<AppState>,
    user_prompt: String,
) -> Result<(HeaderMap, Sse<impl Stream<Item = Result<Event, Infallible>>>), StatusCode> {
    if let Err(e) = sqlx::query("SELECT 1").fetch_one(&state.pool).await {
        eprintln!("Database ping failed: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let model = select();
    println!("Calling {}", model.name());
    let hostname = var("ROULETTE_HOSTNAME").unwrap_or(String::from("http://localhost:5173"));

    if let Ok(stream) = model.call(&user_prompt) {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, hostname.parse().unwrap());

        Ok((
            headers,
            Sse::new(stream.map(move |chunk| Event::default().data(chunk)).map(Ok)).keep_alive(KeepAlive::default()),
        ))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
