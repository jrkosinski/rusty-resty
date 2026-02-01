//! Router utilities for RustAPI framework
//!
//! Provides helper functions to work with routers without exposing Axum directly.
//! Users interact through these functions rather than importing Axum types.

/// Re-export Axum's Router type
///
/// Note: In Axum's type system, `Router<S>` means a router that "needs" state of type S.
/// - `Router<()>` = a stateless router (needs no state)
/// - `Router<AppState>` = a router that needs AppState to be provided via `.with_state()`
pub type Router<S = ()> = axum::Router<S>;

/// Create a new empty router
///
/// This is the starting point for building a router. The router starts stateless.
///
/// # Example
///
/// ```ignore
/// use rustapi_core::router;
///
/// let app = router::new()
///     .route("/", get(handler));
/// ```
pub fn new() -> Router<()> {
    axum::Router::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let _router = new();
    }
}
