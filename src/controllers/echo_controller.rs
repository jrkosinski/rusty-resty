use axum::{extract::State, http::StatusCode, Json};
use serde::{Serialize, Deserialize};
use crate::services::echo_service::{EchoResponse, EchoService};
use std::sync::Arc;

/// Request type for the echo endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequest {
    message: String,
}

#[derive(Clone)]
pub struct EchoController {
    echo_service: Arc<EchoService>
}

impl EchoController
{
    /// Create a new echo controller with injected dependencies
    pub fn new(echo_service: Arc<EchoService>) -> Self {
        Self {
            echo_service
        }
    }

    pub async fn echo(
        State(controller): State<Arc<Self>>,
        Json(payload): Json<EchoRequest>
    ) -> (StatusCode, Json<EchoResponse>) {
        let response = controller.process_echo(&payload.message);
        (StatusCode::OK, Json(response))
    }

    //delegate to the echo service to process the message
    fn process_echo(&self, message: &str) -> EchoResponse {
        self.echo_service.echo(message)
    }
}