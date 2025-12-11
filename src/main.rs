use std::env;
use anyhow::Ok;
use axum::{Router, routing::{get, post}};
use dotenvy;
use tracing_subscriber::EnvFilter;
use std::net::SocketAddr;

use crate::state::AppState;

mod state;
mod models;
mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    // reading .env file
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url = env::var("DATABASE_URL")
        .expect("FAILDED TO FETCH URL FOR DB.");

    let state = AppState::new(&database_url).await?;

    let app = Router::new()
        .route("/shorten", post(handlers::shorten::shorten_url))
        .route("/{code}", get(handlers::redirect::redirect_to_original))
        .with_state(state);

    let bind_addr: SocketAddr = env::var("BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".into())
        .parse()?;

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;

    tracing::info!("Listening on {}", bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}