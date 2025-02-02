pub mod adjacency;
pub mod directional;

use std::collections::HashMap;
use std::rc::Rc;

use crate::utils::{bitset::BitSet, cluster_tracker::ClusterTracker, read_freq::ReadFreq};

pub trait Algorithm {
    fn apply(
        &mut self,
        reads: &HashMap<Rc<BitSet>, Rc<ReadFreq>>,
        tracker: &mut ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<Rc<dyn crate::utils::read::UcRead>>;
}
