use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct LiveRoomRequest {
    uid: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct LiveRoomResponse {
    title: String,
    watched: u32,
    cover: String,
    is_streaming: bool
}