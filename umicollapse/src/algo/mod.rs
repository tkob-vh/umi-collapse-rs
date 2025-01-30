pub mod adjacency;
pub mod directional;

use std::{collections::HashMap, sync::Arc};

use crate::{
    data::DataStruct,
    utils::{bitset::BitSet, cluster_tracker::ClusterTracker, read_freq::ReadFreq},
};

pub trait Algorithm {
    fn apply(
        &self,
        reads: &HashMap<Arc<BitSet>, Arc<ReadFreq>>,
        data: &mut Box<dyn DataStruct>,
        tracker: &mut ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<Arc<dyn crate::utils::read::UcRead>>;
}
