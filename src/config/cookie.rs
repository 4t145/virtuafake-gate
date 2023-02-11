use serde::{Deserialize, Serialize};

use super::EnvConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub cookies: Vec<CookieItem>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CookieItem {
    pub url: String,
    pub value: String
}

impl EnvConfig for Config {
    const ENV: &'static str = "COOKIE";
}