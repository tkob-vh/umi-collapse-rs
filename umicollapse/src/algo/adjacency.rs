#![allow(clippy::mutable_key_type)]

use std::collections::HashMap;

use crate::{
    data::DataStruct,
    utils::{bitset::BitSet, read::UcRead, umi_freq::UmiFreq},
};

use super::Algorithm;

#[allow(dead_code)]
pub struct Adjacency {
    k: i32,
    percentage: f32,
    track_cluster: bool,
}

impl Adjacency {
    pub fn new(args: &crate::cli::Cli) -> Self {
        Self {
            k: args.k,
            percentage: args.percentage,
            track_cluster: args.track_clusters,
        }
    }
}

impl Algorithm for Adjacency {
    #[allow(unused_variables)]
    fn apply<'align, R: UcRead, D: DataStruct<'align>>(
        &mut self,
        reads: &'align HashMap<
            &crate::utils::bitset::BitSet,
            &crate::utils::read_freq::ReadFreq<R>,
        >,
        tracker: &mut crate::utils::cluster_tracker::ClusterTracker<'align, 'align, R>,
        umi_length: usize,
    ) -> Vec<&'align R> {
        let mut freq: Vec<UmiFreq<R>> = reads
            .iter()
            .map(|(umi, rf)| UmiFreq::new(umi, rf))
            .collect();

        freq.sort_by(|a, b| b.read_freq.freq.cmp(&a.read_freq.freq));

        let m: HashMap<&BitSet, i32> = reads.iter().map(|(&umi, rf)| (umi, rf.freq)).collect();

        let mut data: D = D::new(m, umi_length, self.k);
        let mut res: Vec<&R> = Vec::new();

        for entry in freq {
            let umi = entry.umi;
            let read_freq = entry.read_freq;
            if data.contains(umi) {
                tracker.add_all(&data.remove_near(umi, self.k, 0), reads);
                tracker.track(umi, &read_freq.read);
                res.push(&read_freq.read);
            }
        }

        res
    }
}
