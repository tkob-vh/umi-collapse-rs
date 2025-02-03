pub mod adjacency;
pub mod directional;

use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    data::DataStruct,
    utils::{bitset::BitSet, cluster_tracker::ClusterTracker, read::UcRead, read_freq::ReadFreq},
};

pub trait Algorithm {
    fn apply<R: UcRead, D: DataStruct>(
        &mut self,
        reads: &HashMap<Rc<BitSet>, Rc<ReadFreq<R>>>,
        data: &mut D,
        tracker: &mut ClusterTracker<R>,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<Rc<dyn crate::utils::read::UcRead>>;
}
