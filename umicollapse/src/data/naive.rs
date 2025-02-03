//! `Naive` data structure.

#![allow(clippy::mutable_key_type)]

use std::collections::{HashMap, HashSet};

use crate::utils::{bitset::BitSet, umi_dist};

use super::DataStruct;

#[derive(Clone, Default)]
pub struct Naive<'bitset> {
    umi_freq: HashMap<&'bitset BitSet, i32>,
}

impl<'a> DataStruct<'a> for Naive<'a> {
    #[allow(unused_variables)]
    fn re_init(&mut self, umi_freq: HashMap<&'a BitSet, i32>, umi_length: usize, max_edits: i32) {
        self.umi_freq = umi_freq;
    }

    fn remove_near(&mut self, umi: &BitSet, k: i32, max_freq: i32) -> HashSet<&'a BitSet> {
        let mut res: HashSet<&BitSet> = HashSet::new();
        let umi_freq = &mut self.umi_freq;
        umi_freq.retain(|o, &mut f| {
            let dist: i32 = umi_dist(umi, o);
            if dist <= k && (dist == 0 || f <= max_freq) {
                res.insert(o);
                false
            } else {
                true
            }
        });

        res
    }

    fn contains(&self, umi: &BitSet) -> bool {
        self.umi_freq.contains_key(umi)
    }

    fn stats(&self) -> HashMap<String, f32> {
        HashMap::new()
    }
}
