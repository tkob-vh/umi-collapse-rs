use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    data::DataStruct,
    utils::{
        bitset::BitSet, cluster_tracker::ClusterTracker, read_freq::ReadFreq, umi_freq::UmiFreq,
    },
};

use super::Algorithm;

pub struct Directional {}

impl Directional {
    pub fn new() -> Self {
        Self {}
    }

    fn visit_and_remove(
        start_umi: Rc<BitSet>,
        reads: &HashMap<Rc<BitSet>, Rc<ReadFreq>>,
        data: &mut Box<dyn DataStruct>,
        tracker: &mut ClusterTracker,
        k: i32,
        percentage: f32,
    ) {
        // Calculate threshold exactly as Java does
        let threshold = (percentage * (reads.get(&start_umi).unwrap().freq + 1) as f32) as i32;
        let near_umis = data.remove_near(&start_umi, k, threshold);

        // Record to tracker
        tracker.add_all(&near_umis, reads);

        // Recursive DFS, matching Java implementation
        for v in near_umis {
            if v == start_umi {
                continue;
            }

            Directional::visit_and_remove(v, reads, data, tracker, k, percentage);
        }
    }
}

impl Algorithm for Directional {
    fn apply(
        &self,
        reads: &HashMap<Rc<BitSet>, Rc<ReadFreq>>,
        data: &mut Box<dyn DataStruct>,
        tracker: &mut ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<Rc<dyn crate::utils::read::UcRead>> {
        let m: HashMap<Rc<BitSet>, i32> = reads
            .iter()
            .map(|(umi, rf)| (umi.clone(), rf.freq))
            .collect();

        let mut freq: Vec<UmiFreq> = reads
            .iter()
            .map(|(umi, rf)| UmiFreq::new(umi.clone(), rf.clone()))
            .collect();

        freq.sort_by(|a, b| b.read_freq.freq.cmp(&a.read_freq.freq));

        data.change(m, umi_length, k);

        let mut res: Vec<Rc<dyn crate::utils::read::UcRead>> = Vec::new();

        for entry in freq {
            let umi = entry.umi;
            let read_freq = entry.read_freq;
            if data.contains(&umi) {
                Directional::visit_and_remove(umi.clone(), reads, data, tracker, k, percentage);
                tracker.track(umi.clone(), &read_freq.read);
                res.push(read_freq.read.clone());
            }
        }

        res
    }
}
