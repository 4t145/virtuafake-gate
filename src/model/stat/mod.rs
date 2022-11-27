use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Heat {
    pub minute: u32,
    pub danmaku_count: u32
}