use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::server::AppState;

/// GET /voice-channels
/// Returns a snapshot of all active voice channels and their participants.
pub async fn get_voice_channels(State(state): State<AppState>) -> Json<Value> {
    let channels = state.hub.get_voice_channels().await;
    let result: Vec<Value> = channels
        .into_iter()
        .map(|(id, users)| json!({ "id": id, "users": users }))
        .collect();
    Json(json!(result))
}
