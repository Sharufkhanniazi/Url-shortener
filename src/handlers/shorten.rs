
use axum::{Json, extract::State, http::StatusCode};
use rand::{Rng, distributions::Alphanumeric, thread_rng};
use serde_json::json;
use crate::{models::{ShortenRequest, ShortenResponse}, state::AppState};
use url::Url;


const SHORT_CODE_LEN: usize = 6;

pub async fn shorten_url(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>
)-> Result<(StatusCode, Json<ShortenResponse>),(StatusCode, Json<serde_json::Value>)>{
    
    if Url::parse(&payload.url).is_err(){
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "invalid url" }))));
    }

    let code: String = loop {
        let generated: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(SHORT_CODE_LEN)
            .map(char::from)
            .collect();

        let exist = sqlx::query_as::<_,(i64,)>(
            "SELECT id FROM urls WHERE short_code = $1"
        )
        .bind(&generated)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db error"})))
        })?;

        if exist.is_none(){
            break generated;
        }
    };

    sqlx::query("INSERT INTO urls (short_code, original_url) VALUES ($1, $2)")
        .bind(&code)
        .bind(&payload.url)
        .execute(&state.db)
        .await
        .map_err(|_| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error" : "Failed to insert"})))
        })?;
    
    let base = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let short_url = format!("{}/{}", base.trim_end_matches('/'), code);

    Ok((StatusCode::CREATED, Json(ShortenResponse { short_url })))
}