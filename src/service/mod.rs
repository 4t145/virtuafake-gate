mod index;
pub mod liver;
pub mod liveroom;
pub use index::index_page;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Paged<T> 
{
    pub data: Vec<T>,
    pub pagination: Pagination
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    pub size: u64,
    #[serde(default)]
    pub page: u64,
    #[serde(default)]
    pub total: u64,
}

impl Pagination {
    pub fn as_mongodb_option(self) -> mongodb::options::FindOptions {
        let limit = Some(self.size as i64);
        let skip = Some((self.size * self.page) as u64);
        mongodb::options::FindOptions::builder().limit(limit).skip(skip).build()
    }
}