use axum::{
    routing::{get, post},
    Router,
};

use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rusty_resty_core::Container;

mod controllers;
mod services;

use controllers::health_controller::HealthController;
use controllers::echo_controller::EchoController;
use services::health_service::HealthService;
use services::echo_service::EchoService;

/// Main entry point for the rusty-resty REST API server.
/// Demonstrates the DI container pattern inspired by NestJS/FastAPI.
#[tokio::main]
async fn main() {
    initialize_tracing();
    let container = setup_container();
    let app = build_router(&container);
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

/// Sets up the DI container with all services
fn setup_container() -> Container {
    let mut container = Container::new();

    //register services
    container.register_factory(|| HealthService::new());
    container.register_factory(|| EchoService::new());

    container
}

/// Builds the application router with all routes and middleware
fn build_router(container: &Container) -> Router {
    //resolve services from container
    let health_service = container.resolve::<HealthService>().unwrap();
    let echo_service = container.resolve::<EchoService>().unwrap();

    //initialize controllers with injected dependencies
    let health_controller = Arc::new(HealthController::new(health_service));
    let echo_controller = Arc::new(EchoController::new(echo_service));

    //create nested routers with individual states
    let health_router = create_health_router(health_controller);
    let echo_router = create_echo_router(echo_controller);

    //merge all routers together
    Router::new()
        .route("/", get(root))
        .merge(health_router)
        .merge(echo_router)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

//create the health check router
fn create_health_router(controller: Arc<HealthController>) -> Router {
    Router::new()
        .route("/health", get(HealthController::health_check))
        .with_state(controller)
}

//create the echo router
fn create_echo_router(controller: Arc<EchoController>) -> Router {
    Router::new()
        .route("/echo", post(EchoController::echo))
        .with_state(controller)
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
