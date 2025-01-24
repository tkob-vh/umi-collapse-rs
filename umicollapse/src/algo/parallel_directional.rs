use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::utils::{bitset::BitSet, read_freq::ReadFreq};

use super::{Algo, ParallelAlgorithm};

pub struct ParallelDirectional {}

impl ParallelDirectional {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    fn visit_and_remove(
        u: &BitSet,
        reads: &HashMap<BitSet, ReadFreq>,
        adj: &HashMap<BitSet, HashSet<BitSet>>,
        visited: &mut HashSet<BitSet>,
    ) {
        if visited.contains(u) {
            return;
        }

        let c: &HashSet<BitSet> = adj.get(u).expect("Failed to get HashSet<BitSet>");
        visited.insert(u.clone());

        for v in c {
            if u.eq(v) {
                continue;
            }

            ParallelDirectional::visit_and_remove(v, reads, adj, visited);
        }
    }
}

impl Algo for ParallelDirectional {}

impl ParallelAlgorithm for ParallelDirectional {
    #[allow(unused_variables)]
    fn apply(
        &self,
        reads: &HashMap<BitSet, ReadFreq>,
        data: &mut Box<dyn crate::data::ParallelDataStruct>,
        tracker: &mut crate::utils::cluster_tracker::ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<Arc<dyn crate::utils::read::UcRead>> {
        let mut _res: Vec<Arc<dyn crate::utils::read::UcRead>> = Vec::new();
        _res
    }
}
