pub mod adjacency;
pub mod directional;
pub mod parallel_adjacency;
pub mod parallel_directional;

use std::{collections::HashMap, sync::Arc};

use crate::{
    data::{DataStruct, ParallelDataStruct},
    utils::{bitset::BitSet, cluster_tracker::ClusterTracker, read_freq::ReadFreq},
};

pub trait Algo {}

pub trait Algorithm: Algo {
    fn apply(
        reads: &HashMap<Arc<BitSet>, Arc<ReadFreq>>,
        data: &mut Box<dyn DataStruct>,
        tracker: &mut ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<Arc<dyn crate::utils::read::UcRead>>;
}

#[allow(dead_code)]
pub trait ParallelAlgorithm: Algo {
    fn apply(
        &self,
        reads: &HashMap<BitSet, ReadFreq>,
        data: &mut Box<dyn ParallelDataStruct>,
        tracker: &mut ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<Arc<dyn crate::utils::read::UcRead>>;
}
