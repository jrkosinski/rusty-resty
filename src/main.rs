use axum::{routing::{get, post}, Router};
use rustapi_core::Container;
use rustapi_macros::get as get_macro;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod services;

// Import the controller handler functions directly
use controllers::echo_controller::echo;
use controllers::health_controller::health_check;
use services::echo_service::EchoService;
use services::health_service::HealthService;

/// Root endpoint handler that returns a welcome message.
#[get_macro("/")]
async fn root() -> &'static str {
    "Welcome to RustAPI!"
}

/// Main entry point for the rustapi REST API server.
/// Demonstrates FastAPI-style routing with decorator macros and dependency injection.
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
                .unwrap_or_else(|_| "rustapi=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Sets up the DI container with all services
fn setup_container() -> Container {
    let mut container = Container::new();

    // Register services
    container.register_factory(|| HealthService::new());
    container.register_factory(|| EchoService::new());

    container
}

/// Builds the application router using FastAPI-style route decorators
/// Routes with State need to be added with `.with_state()` for DI
fn build_router(container: &Container) -> Router {
    // Resolve services from container
    let health_service = container.resolve::<HealthService>().unwrap();
    let echo_service = container.resolve::<EchoService>().unwrap();

    // Build separate routers for each service with their own state
    let health_router = Router::new()
        .route("/health", get(health_check))
        .with_state(health_service);

    let echo_router = Router::new()
        .route("/echo", post(echo))
        .with_state(echo_service);

    // Merge all routers together
    Router::new()
        .route(__root_route.0, __root_route.1())
        .merge(health_router)
        .merge(echo_router)
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
