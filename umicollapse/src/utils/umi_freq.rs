use std::rc::Rc;

use crate::utils;

use super::read::UcRead;

#[allow(dead_code)]
pub struct UmiFreq<R: UcRead> {
    pub umi: Rc<utils::bitset::BitSet>,
    pub read_freq: Rc<utils::read_freq::ReadFreq<R>>,
}

impl<R: UcRead> UmiFreq<R> {
    #[allow(dead_code)]
    pub fn new(
        umi: Rc<utils::bitset::BitSet>,
        read_freq: Rc<utils::read_freq::ReadFreq<R>>,
    ) -> Self {
        Self { umi, read_freq }
    }
}
