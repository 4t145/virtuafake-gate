use serde::{Deserialize, Serialize};

pub fn default_color() -> String {
    return "#ffffff".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Liver {
    /// 备注
    pub remark: Option<String>,
    pub uid: u64,
    /// 分组
    pub group: Group,
    /// 应援色
    #[serde(default="default_color")]
    pub color: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag="tag", rename_all="lowercase")]
/// 分组
pub enum Group {
    /// 梦魇tsuki专属的
    Tsuki,
    /// 正维旗的
    Project {
        /// 年功序列的
        gen: u8
    },
    /// 臭link的
    Link,
    /// 带明星的
    Star
}