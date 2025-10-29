use axum::{
    routing::get,
    Router,
};
use futures::StreamExt;

use crate::models::model::select;

pub fn app() -> Router {
    Router::new().route("/prompt", get(prompt))
}

#[axum::debug_handler]
async fn prompt() -> String {
    let model = select();

    if let Ok(mut stream) = model.call("Calling a model!") {
        while let Some(chunk) = stream.next().await {
            println!("{chunk:?}");
        }
    }


    format!("Called {}", model.name())
}