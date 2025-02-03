#![allow(clippy::mutable_key_type)]

use std::collections::HashMap;

use crate::{
    data::DataStruct,
    utils::{
        bitset::BitSet, cluster_tracker::ClusterTracker, read::UcRead, read_freq::ReadFreq,
        umi_freq::UmiFreq,
    },
};

use super::Algorithm;

pub struct Directional {}

impl Directional {
    pub fn new() -> Self {
        Self {}
    }
    fn visit_and_remove<'align, R: UcRead, D: DataStruct<'align>>(
        start_umi: &BitSet,
        reads: &HashMap<BitSet, ReadFreq<R>>,
        data: &mut D,
        tracker: &mut ClusterTracker<'align, 'align, R>,
        k: i32,
        percentage: f32,
    ) {
        // Calculate threshold exactly as Java does
        let threshold = (percentage * (reads.get(start_umi).unwrap().freq + 1) as f32) as i32;
        let near_umis = data.remove_near(start_umi, k, threshold);

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
    fn apply<'align, R: UcRead, D: DataStruct<'align>>(
        &mut self,
        reads: &'align HashMap<BitSet, ReadFreq<R>>,
        tracker: &mut ClusterTracker<'align, 'align, R>,
        umi_length: usize,
        k: i32,
        percentage: f32,
    ) -> Vec<&'align R> {
        let data_member: HashMap<&BitSet, i32> =
            reads.iter().map(|(umi, rf)| (umi, rf.freq)).collect();

        let mut umi_freqs: Vec<UmiFreq<R>> = reads
            .iter()
            .map(|(umi, rf)| UmiFreq::new(umi, rf))
            .collect();

        umi_freqs.sort_by(|a, b| b.read_freq.freq.cmp(&a.read_freq.freq));

        let mut data: D = D::default();
        data.re_init(data_member, umi_length, k);

        let mut res: Vec<&R> = Vec::new();

        for entry in umi_freqs {
            let umi = entry.umi;
            let read_freq = entry.read_freq;
            if data.contains(umi) {
                Directional::visit_and_remove(umi, reads, &mut data, tracker, k, percentage);
                tracker.track(umi, &read_freq.read);
                res.push(&read_freq.read);
            }
        }

        res
    }
}
