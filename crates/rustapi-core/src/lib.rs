//! RustAPI Core
//!
//! Core runtime for the rustapi framework, providing:
//! - Dependency injection container
//! - Application builder
//! - Service lifecycle management
//! - HTTP server runtime
//! - Router utilities (wrapping Axum)

pub mod di;
pub mod app;
pub mod error;
pub mod server;
pub mod router;

pub use di::{Container, Injectable};
pub use app::App;
pub use error::{Error, Result};
pub use server::RustAPI;
pub use router::Router;

// Re-export routing methods from Axum
// These are used to define route handlers (get, post, put, delete, etc.)
pub mod routing {
    pub use axum::routing::*;
}

// Re-export common middleware layers
pub use tower_http::cors::CorsLayer;
pub use tower_http::trace::TraceLayer;
