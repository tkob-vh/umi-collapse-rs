use std::rc::Rc;

use crate::utils;

#[allow(dead_code)]
pub struct UmiFreq {
    pub umi: Rc<utils::bitset::BitSet>,
    pub read_freq: Rc<utils::read_freq::ReadFreq>,
}

impl UmiFreq {
    #[allow(dead_code)]
    pub fn new(umi: Rc<utils::bitset::BitSet>, read_freq: Rc<utils::read_freq::ReadFreq>) -> Self {
        Self { umi, read_freq }
    }
}
