//! Health check

use std::collections::HashMap;

use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::Connection;

use super::{ServiceError, ServiceState};

/// Health check HTTP handler.
pub async fn health_check(
    State(state): State<ServiceState>,
) -> Result<Json<ServerStatus>, ServiceError> {
    let db_status = if let Ok(mut conn) = state.db_pool.acquire().await {
        conn.ping().await.is_ok()
    } else {
        false
    };

    let dependencies = [("database".to_string(), db_status)].into_iter().collect();

    Ok(Json(ServerStatus {
        status: "ok".to_string(),
        ok: true,
        dependencies,
    }))
}

/// The response for the health check endpoint.
#[derive(Serialize)]
pub struct ServerStatus {
    status: String,
    ok: bool,
    dependencies: HashMap<String, bool>,
}
