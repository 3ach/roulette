use axum::{
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::{post, get_service},
    Router,
};
use futures::{Stream, StreamExt};
use std::convert::Infallible;
use tower_http::services::ServeDir;

use crate::models::model::select;

pub fn app() -> Router {
    let static_route = Router::new().nest_service("/frontend", get_service(ServeDir::new("./frontend")));
    Router::new().route("/prompt", post(prompt))
    .merge(static_route)
}

async fn prompt(user_prompt: String) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let model = select();
    println!("Calling {}", model.name());

    if let Ok(stream) = model.call(&user_prompt) {
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
