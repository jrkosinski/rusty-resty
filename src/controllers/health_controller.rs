use axum::{extract::State, http::StatusCode, Json};
use crate::services::health_service::{HealthService, HealthResponse};
use std::sync::Arc;

/// Health controller that manages health check endpoints.
/// This struct can hold shared state, dependencies, and provides lifecycle management.
#[derive(Clone)]
pub struct HealthController {
    health_service: Arc<HealthService>,
}

impl HealthController {
    /// Create a new health controller with injected dependencies
    pub fn new(health_service: Arc<HealthService>) -> Self {
        Self {
            health_service,
        }
    }

    /// Health check endpoint that returns the service status.
    /// Returns a 200 OK status with a JSON response indicating the service is healthy.
    /// This is a pass-through to the health service.
    pub async fn health_check(
        State(controller): State<Arc<Self>>
    ) -> (StatusCode, Json<HealthResponse>) {
        let response = controller.check_health();
        (StatusCode::OK, Json(response))
    }

    //delegate to the health service to perform the check
    fn check_health(&self) -> HealthResponse {
        self.health_service.health_check()
    }
}
