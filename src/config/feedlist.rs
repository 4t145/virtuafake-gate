use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::model::liver::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feedlist {
    pub liver: Vec<Liver>
}
impl Feedlist {
    pub fn load_from_env() -> Self {
        let path = env!("FEEDLIST");
        use std::fs::*;
        let mut fl: Self = toml::from_slice(&read(path).unwrap()).unwrap();
        fl.liver.sort_by_key(|k|k.uid);
        fl
    }

}