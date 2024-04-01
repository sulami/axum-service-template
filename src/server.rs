//! HTTP server

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use color_eyre::{
    eyre::{Report, WrapErr},
    Result,
};
use tokio::{net::TcpListener, select, signal};
use tower_http::trace::TraceLayer;
use tracing::info;

mod health;
mod index;

/// Runs the HTTP server on the specified port.
pub async fn run(port: u16) -> Result<()> {
    let app = Router::new()
        .route("/", get(index::index))
        .route("/health-check", get(health::health_check))
        // NB Add new routes here.
        .layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .wrap_err("failed to bind to port")?;
    info!("listening on http://0.0.0.0:{port}");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .wrap_err("server failed")?;

    Ok(())
}

/// A service error, resulting in a 500 response.
struct ServiceError(Report);

impl<E> From<E> for ServiceError
where
    E: Into<Report>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
            .into_response()
    }
}

/// Shutdown signal. When this future returns, the server will start shutting down.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
