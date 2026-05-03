use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Ping,
    SubscribeGroup { group_id: i64 },
    UnsubscribeGroup { group_id: i64 },
    Typing { group_id: i64 },
    Presence { state: String },
    // Voice channel signaling
    VoiceJoin { channel_id: String },
    VoiceLeave { channel_id: String },
    VoiceOffer { channel_id: String, target: String, sdp: String },
    VoiceAnswer { channel_id: String, target: String, sdp: String },
    // candidate is a JSON object { candidate, sdpMid, sdpMLineIndex }
    VoiceIce { channel_id: String, target: String, candidate: serde_json::Value },
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Pong,
    InitComplete,
    Error { message: String },
    // Voice channel events
    VoiceChannelState { channel_id: String, users: Vec<String> },
    VoiceUserJoined { channel_id: String, username: String },
    VoiceUserLeft { channel_id: String, username: String },
    VoiceOffer { channel_id: String, from: String, sdp: String },
    VoiceAnswer { channel_id: String, from: String, sdp: String },
    VoiceIce { channel_id: String, from: String, candidate: serde_json::Value },
}
