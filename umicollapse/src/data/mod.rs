//! The data structures used in this project.

pub mod combo;
pub mod naive;

pub mod parallel_naive;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use downcast_rs::{impl_downcast, Downcast};

use crate::utils::bitset::BitSet;

pub trait Data {}

pub trait DataStruct: Data + Downcast {
    fn change(&mut self, umi_freq: HashMap<Arc<BitSet>, i32>, umi_length: usize, max_edits: i32);
    fn remove_near(&mut self, umi: &BitSet, k: i32, max_freq: i32) -> HashSet<Arc<BitSet>>;
    fn contains(&self, umi: &BitSet) -> bool;
    #[allow(dead_code)]
    fn stats(&self) -> HashMap<String, f32>;
}
impl_downcast!(DataStruct);

pub trait ParallelDataStruct: Data + Downcast {
    #[allow(dead_code)]
    fn change(&mut self, umi_freq: HashMap<BitSet, i32>, umi_length: usize, max_edits: i32);
    #[allow(dead_code)]
    fn near(&self, umi: &BitSet, k: i32, max_freq: i32) -> HashSet<BitSet>;
}
impl_downcast!(ParallelDataStruct);
