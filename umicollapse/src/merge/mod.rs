//! The merge algorithms.
//!

use crate::utils::read::{UcRead, UcSAMRead};

pub trait Merge<R: UcRead> {
    fn merge(&self, a: &R, b: &R) -> bool;
}

pub struct AnyMerge;

impl AnyMerge {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: UcRead> Merge<R> for AnyMerge {
    #[allow(unused_variables)]
    fn merge(&self, a: &R, b: &R) -> bool {
        true
    }
}

pub struct AvgQualMerge;

impl AvgQualMerge {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: UcRead> Merge<R> for AvgQualMerge {
    fn merge(&self, a: &R, b: &R) -> bool {
        a.get_avg_qual() >= b.get_avg_qual()
    }
}

pub struct MapQualMerge;

impl MapQualMerge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Merge<UcSAMRead> for MapQualMerge {
    fn merge(&self, a: &UcSAMRead, b: &UcSAMRead) -> bool {
        a.get_map_qual() >= b.get_map_qual()
    }
}
