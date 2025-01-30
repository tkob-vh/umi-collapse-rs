pub mod adjacency;
pub mod directional;

use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    data::DataStruct,
    utils::{bitset::BitSet, cluster_tracker::ClusterTracker, read_freq::ReadFreq},
};

pub trait Algorithm {
    fn apply(
        &self,
        reads: &HashMap<Rc<BitSet>, Rc<ReadFreq>>,
        data: &mut Box<dyn DataStruct>,
        tracker: &mut ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<Rc<dyn crate::utils::read::UcRead>>;
}
