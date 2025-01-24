use std::{collections::HashMap, sync::Arc};

use crate::{
    data::DataStruct,
    utils::{
        bitset::BitSet, cluster_tracker::ClusterTracker, read::UcSAMRead, read_freq::ReadFreq,
        umi_freq::UmiFreq,
    },
};

use super::{Algo, Algorithm};

pub struct Directional {}

impl Directional {
    pub fn new() -> Self {
        Self {}
    }

    fn visit_and_remove(
        start_umi: Arc<BitSet>,
        reads: &HashMap<Arc<BitSet>, Arc<ReadFreq>>,
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

impl Algo for Directional {}

impl Algorithm for Directional {
    fn apply(
        reads: &HashMap<Arc<BitSet>, Arc<ReadFreq>>,
        data: &mut Box<dyn DataStruct>,
        tracker: &mut ClusterTracker,
        umi_length: usize,
        k: i32,
        percentage: f32,
        primary_count: &mut i32,
        secondary_count: &mut i32,
    ) -> Vec<Arc<dyn crate::utils::read::UcRead>> {
        let m: HashMap<Arc<BitSet>, i32> = reads
            .iter()
            .map(|(umi, rf)| (umi.clone(), rf.freq))
            .collect();

        let mut freq: Vec<UmiFreq> = reads
            .iter()
            .enumerate() // 添加索引以保持原始顺序
            .map(|(_idx, (umi, rf))| UmiFreq::new(umi.clone(), rf.clone()))
            .collect();

        freq.sort_by(|a, b| b.read_freq.freq.cmp(&a.read_freq.freq));

        data.change(m, umi_length, k);

        let mut res: Vec<Arc<dyn crate::utils::read::UcRead>> = Vec::new();

        for entry in freq {
            let umi = entry.umi;
            let read_freq = entry.read_freq;
            if data.contains(&umi) {
                if read_freq
                    .read
                    .downcast_ref::<UcSAMRead>()
                    .unwrap()
                    .to_sam_record()
                    .is_secondary()
                {
                    *secondary_count += 1;
                } else {
                    *primary_count += 1;
                }
                Directional::visit_and_remove(umi.clone(), reads, data, tracker, k, percentage);
                tracker.track(umi.clone(), &read_freq.read);
                res.push(read_freq.read.clone());
            }
        }

        res
    }
}
