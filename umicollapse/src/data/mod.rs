//! The data structures used in this project.

pub mod combo;
pub mod naive;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use downcast_rs::{impl_downcast, Downcast};

use crate::utils::bitset::BitSet;

pub trait DataStruct: Downcast {
    fn change(&mut self, umi_freq: HashMap<Rc<BitSet>, i32>, umi_length: usize, max_edits: i32);
    fn remove_near(&mut self, umi: &BitSet, k: i32, max_freq: i32) -> HashSet<Rc<BitSet>>;
    fn contains(&self, umi: &BitSet) -> bool;
    #[allow(dead_code)]
    fn stats(&self) -> HashMap<String, f32>;
}
impl_downcast!(DataStruct);
