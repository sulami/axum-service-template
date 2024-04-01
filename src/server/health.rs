//! Health check

use axum::Json;
use serde::Serialize;

use super::ServiceError;

/// Health check HTTP handler.
pub async fn health_check() -> Result<Json<ServerStatus>, ServiceError> {
    Ok(Json(ServerStatus {
        status: "ok".to_string(),
    }))
}

/// The response for the health check endpoint.
#[derive(Serialize)]
pub struct ServerStatus {
    status: String,
}
