use serde::{Deserialize, Serialize};

use super::EnvConfig;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub db: String,
}

impl EnvConfig for Config {
    const ENV: &'static str = "MONGODB";
}