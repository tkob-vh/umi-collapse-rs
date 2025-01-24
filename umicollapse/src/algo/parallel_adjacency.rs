use std::sync::Arc;

use super::{Algo, ParallelAlgorithm};

pub struct ParallelAdjacency {}

impl ParallelAdjacency {
    pub fn new() -> Self {
        Self {}
    }
}

impl Algo for ParallelAdjacency {}

impl ParallelAlgorithm for ParallelAdjacency {
    #[allow(unused_variables)]
    fn apply(
        &self,
        reads: &std::collections::HashMap<
            crate::utils::bitset::BitSet,
            crate::utils::read_freq::ReadFreq,
        >,
        data: &mut Box<dyn crate::data::ParallelDataStruct>,
        tracker: &mut crate::utils::cluster_tracker::ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<std::sync::Arc<dyn crate::utils::read::UcRead>> {
        let mut _res: Vec<Arc<dyn crate::utils::read::UcRead>> = Vec::new();
        _res
    }
}
