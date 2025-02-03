use super::{bitset::BitSet, read::UcRead, read_freq::ReadFreq};

#[allow(dead_code)]
pub struct UmiFreq<'umi_freq, R: UcRead> {
    pub umi: &'umi_freq BitSet,
    pub read_freq: &'umi_freq ReadFreq<R>,
}

impl<'a, R: UcRead> UmiFreq<'a, R> {
    #[allow(dead_code)]
    pub fn new(umi: &'a BitSet, read_freq: &'a ReadFreq<R>) -> Self {
        Self { umi, read_freq }
    }
}
