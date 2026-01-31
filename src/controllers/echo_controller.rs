use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::echo_service::{EchoService, EchoResponse};

/// Request type for the echo endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

/// Echo endpoint that echoes back the received message.
/// Uses dependency injection to access the EchoService.
pub async fn echo(
    State(service): State<Arc<EchoService>>,
    Json(payload): Json<EchoRequest>
) -> Json<EchoResponse> {
    let response = service.echo(&payload.message);
    Json(response)
}