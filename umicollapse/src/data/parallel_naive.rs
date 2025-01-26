use std::collections::{HashMap, HashSet};

use crate::utils::{bitset::BitSet, umi_dist};

use super::{Data, ParallelDataStruct};

pub struct ParallelNaive {
    umi_freq: HashMap<BitSet, i32>,
}

impl ParallelNaive {
    pub fn new() -> Self {
        Self {
            umi_freq: HashMap::new(),
        }
    }
}

impl Data for ParallelNaive {}

impl ParallelDataStruct for ParallelNaive {
    #[allow(unused_variables)]
    fn change(&mut self, umi_freq: HashMap<BitSet, i32>, umi_length: usize, max_edits: i32) {
        self.umi_freq = umi_freq;
    }

    fn near(&self, umi: &BitSet, k: i32, max_freq: i32) -> std::collections::HashSet<BitSet> {
        let mut res: HashSet<BitSet> = HashSet::new();

        for (o, &f) in &self.umi_freq {
            let dist: i32 = umi_dist(umi, o);
            if dist <= k && (dist == 0 || f <= max_freq) {
                res.insert(o.clone());
            }
        }
        res
    }
}
