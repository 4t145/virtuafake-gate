use serde::{Deserialize};

pub mod feedlist;
pub mod mongodb;
pub mod cookie;
pub trait EnvConfig: for <'de> Deserialize<'de> {
    const ENV: &'static str;
    fn load() -> Self where Self:Sized {
        let path = std::env::var(Self::ENV).unwrap();
        use std::fs::*;
        let file = read(path).unwrap();
        let config: Self = toml::from_slice(&file).unwrap();
        config
    }
}