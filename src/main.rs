use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Main entry point for the rusty-resty REST API server.
/// Initializes logging, sets up routes, and starts the HTTP server.
#[tokio::main]
async fn main() {
    initialize_tracing();
    let app = build_router();
    let listener = create_listener().await;
    run_server(listener, app).await;
}

/// Initializes the tracing subscriber for logging
fn initialize_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusty_resty=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Builds the application router with all routes and middleware
fn build_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/echo", post(echo))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

/// Creates and binds the TCP listener on port 3000
async fn create_listener() -> tokio::net::TcpListener {
    tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap()
}

/// Runs the server with the given listener and application router
async fn run_server(listener: tokio::net::TcpListener, app: Router) {
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app)
        .await
        .unwrap();
}

/// Root endpoint handler that returns a welcome message.
async fn root() -> &'static str {
    "Welcome to rusty-resty!"
}

/// Health check endpoint that returns the service status.
/// Returns a 200 OK status with a JSON response indicating the service is healthy.
async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "healthy".to_string(),
        }),
    )
}

/// Echo endpoint that returns the same message that was sent.
/// Accepts a JSON payload and echoes it back to the caller.
async fn echo(Json(payload): Json<EchoRequest>) -> (StatusCode, Json<EchoResponse>) {
    (
        StatusCode::OK,
        Json(EchoResponse {
            message: payload.message,
        }),
    )
}

/// Response type for the health check endpoint.
#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
}

/// Request type for the echo endpoint.
#[derive(Debug, Serialize, Deserialize)]
struct EchoRequest {
    message: String,
}

/// Response type for the echo endpoint.
#[derive(Debug, Serialize, Deserialize)]
struct EchoResponse {
    message: String,
}
