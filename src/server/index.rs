//! Example HTTP handler

use axum::Json;
use serde::Serialize;

use super::ServiceError;

/// The index HTTP handler.
pub async fn index() -> Result<Json<Greeting>, ServiceError> {
    Ok(Json(Greeting {
        message: "I'm sorry Dave, I'm afraid I can't do that.".to_string(),
    }))
}

/// A friendly greeting message.
#[derive(Serialize)]
pub struct Greeting {
    message: String,
}
