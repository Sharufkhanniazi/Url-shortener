use axum::{extract::{Path, State}, http::StatusCode, response::Redirect};

use crate::state::AppState;



pub async fn redirect_to_original(
    State(state): State<AppState>,
    Path(code): Path<String>
)-> Result<Redirect, StatusCode>{

    let row = sqlx::query_as::<_, (String,)>(
        "SELECT original_url FROM urls WHERE short_code = $1"
    )
    .bind(&code)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    
    match row {
        Some((original,)) => {
            let _ = sqlx::query("UPDATE urls SET hits = hits + 1 WHERE short_code = $1")
                .bind(&code)
                .execute(&state.db)
                .await;

            Ok(Redirect::temporary(&original))
        }
        None => Err(StatusCode::NOT_FOUND),
    }

}