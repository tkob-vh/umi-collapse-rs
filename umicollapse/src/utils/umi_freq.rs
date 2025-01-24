use std::sync::Arc;

use crate::utils;

#[allow(dead_code)]
pub struct UmiFreq {
    pub umi: Arc<utils::bitset::BitSet>,
    pub read_freq: Arc<utils::read_freq::ReadFreq>,
}

impl UmiFreq {
    #[allow(dead_code)]
    pub fn new(
        umi: Arc<utils::bitset::BitSet>,
        read_freq: Arc<utils::read_freq::ReadFreq>,
    ) -> Self {
        Self { umi, read_freq }
    }
}
