use std::collections::HashMap;
use std::rc::Rc;

use crate::utils::{bitset::BitSet, umi_freq::UmiFreq};

use super::Algorithm;

pub struct Adjacency {}

impl Adjacency {
    pub fn new() -> Self {
        Self {}
    }
}

impl Algorithm for Adjacency {
    #[allow(unused_variables)]
    fn apply(
        &self,
        reads: &std::collections::HashMap<
            Rc<crate::utils::bitset::BitSet>,
            Rc<crate::utils::read_freq::ReadFreq>,
        >,
        data: &mut Box<dyn crate::data::DataStruct>,
        tracker: &mut crate::utils::cluster_tracker::ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<Rc<dyn crate::utils::read::UcRead>> {
        let mut freq: Vec<UmiFreq> = reads
            .iter()
            .map(|(umi, rf)| UmiFreq::new(umi.clone(), rf.clone()))
            .collect();

        freq.sort_by(|a, b| b.read_freq.freq.cmp(&a.read_freq.freq));

        let m: HashMap<Rc<BitSet>, i32> = reads
            .iter()
            .map(|(umi, rf)| (umi.clone(), rf.freq))
            .collect();

        data.change(m, umi_length, k);
        let mut res: Vec<Rc<dyn crate::utils::read::UcRead>> = Vec::new();

        for entry in freq {
            let umi = entry.umi;
            let read_freq = entry.read_freq;
            if data.contains(&umi) {
                tracker.add_all(&data.remove_near(&umi, k, 0), reads);
                tracker.track(umi, &read_freq.read);
                res.push(read_freq.read.clone());
            }
        }

        res
    }
}
