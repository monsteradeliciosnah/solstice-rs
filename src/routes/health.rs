use axum::{Json, response::IntoResponse};
use serde_json::json;
use utoipa::path;

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "OK")
    )
)]
pub async fn health() -> impl IntoResponse {
    Json(json!({"status":"ok"}))
}
