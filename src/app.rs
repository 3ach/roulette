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
use std::net::IpAddr;
use uuid::Uuid;

use crate::db;
use crate::models::model::select;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

pub async fn app() -> Router {
    let pool = db::pool().await.unwrap();
    let state = AppState { pool };

    Router::new()
        .route("/prompt", post(prompt))
        .with_state(state)
}

async fn prompt(
    State(state): State<AppState>,
    headers: HeaderMap,
    user_prompt: String,
) -> Result<
    (
        HeaderMap,
        Sse<impl Stream<Item = Result<Event, Infallible>>>,
    ),
    StatusCode,
> {
    if let Err(e) = sqlx::query("SELECT 1").fetch_one(&state.pool).await {
        eprintln!("Database ping failed: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mut ipv4 = None;
    let mut ipv6 = None;
    if let Some(x_forwarded_for) = headers.get("x-forwarded-for") {
        match IpAddr::parse_ascii(x_forwarded_for.as_bytes()) {
            Ok(IpAddr::V4(addr)) => ipv4 = Some(addr),
            Ok(IpAddr::V6(addr)) => ipv6 = Some(addr),
            _ => {}, 
        }
    }

    let id = Uuid::new_v4();

    let model = select();

    let picked = model.name().chars().nth(0).unwrap().to_string();

    if let Err(e) = sqlx::query("INSERT INTO turn (id, ipv4, ipv6, model, time) VALUES (?, ?, ?, ?, NOW())")
        .bind(&id.as_bytes()[..])
        .bind(ipv4)
        .bind(ipv6)
        .bind(picked)
        .execute(&state.pool).await {
        eprintln!("Database ping failed: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let hostname = var("ROULETTE_HOSTNAME").unwrap_or(String::from("http://localhost:5173"));

    if let Ok(output) = model.call(&user_prompt) {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            hostname.parse().unwrap(),
        );

        headers.insert(
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            "X-Roulette-Request".parse().unwrap(),
        );

        headers.insert(
            "X-Roulette-Request",
            id.to_string().parse().unwrap()
        );

        Ok((
            headers,
            Sse::new(
                output 
                    .map(move |chunk| Event::default().data(chunk))
                    .map(Ok),
            )
            .keep_alive(KeepAlive::default()),
        ))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
