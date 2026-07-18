use axum::Json;

use crate::services::health;

pub async fn health() -> Json<health::HealthResponse> {
    Json(health::health())
}
