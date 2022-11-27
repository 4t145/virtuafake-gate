#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Event {
    #[serde(flatten)]
    pub event: bilive_danmaku::event::Event,
    pub timestamp: i64
}
