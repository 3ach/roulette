use axum::{
    http::{header, HeaderMap, StatusCode},
    response::sse::{Event, KeepAlive, Sse},
    routing::post,
    Router,
};
use futures::{Stream, StreamExt};
use std::convert::Infallible;
use std::env::var;

use crate::models::model::select;

pub fn app() -> Router {
    Router::new().route("/prompt", post(prompt))
}

async fn prompt(user_prompt: String) -> Result<(HeaderMap, Sse<impl Stream<Item = Result<Event, Infallible>>>), StatusCode> {
    let model = select();
    println!("Calling {}", model.name());
    let hostname = var("ROULETTE_HOSTNAME").unwrap_or(String::from("http://localhost:5173"));

    if let Ok(stream) = model.call(&user_prompt) {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, hostname.parse().unwrap());

        Ok((headers, Sse::new(
            stream
                .map(move |chunk| Event::default().data(chunk))
                .map(Ok),
        )
        .keep_alive(KeepAlive::default())))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
