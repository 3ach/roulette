use anyhow;
use axum::{
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
    Router,
};
use futures::{Stream, StreamExt};
use std::convert::Infallible;

use crate::models::model::select;

pub fn app() -> Router {
    Router::new().route("/prompt", get(prompt))
}

#[axum::debug_handler]
async fn prompt() -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let model = select();

    if let Ok(mut stream) = model.call("Tell me about the French Revolution.") {
        Ok(Sse::new(
            stream
                .map(move |chunk| Event::default().data(chunk))
                .map(Ok),
        )
        .keep_alive(KeepAlive::default()))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
