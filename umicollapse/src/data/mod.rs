//! The data structures used in this project.
#![allow(clippy::mutable_key_type)]

pub mod combo;
pub mod naive;

use std::collections::{HashMap, HashSet};

use crate::utils::bitset::BitSet;

pub trait DataStruct<'a>: Default {
    fn new(umi_freq: HashMap<&'a BitSet, i32>, umi_length: usize, max_edits: i32) -> Self;
    fn remove_near(&mut self, umi: &BitSet, k: i32, max_freq: i32) -> HashSet<&'a BitSet>;
    fn contains(&self, umi: &BitSet) -> bool;
    #[allow(dead_code)]
    fn stats(&self) -> HashMap<String, f32>;
}
