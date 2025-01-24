use std::sync::Arc;

use crate::utils;

#[derive(Debug, Clone)]
pub struct ReadFreq {
    pub read: Arc<dyn utils::read::UcRead>,
    pub freq: i32,
}

impl ReadFreq {
    pub fn new(read: Arc<dyn utils::read::UcRead>, freq: i32) -> Self {
        Self { read, freq }
    }
}
