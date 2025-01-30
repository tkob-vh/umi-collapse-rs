use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::utils::{bitset::BitSet, char_equals, char_set};

use super::DataStruct;

pub struct Combo {
    umi_freq: HashMap<Arc<BitSet>, i32>,
    umi_length: usize,
}

impl Combo {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            umi_freq: HashMap::new(),
            umi_length: 0,
        }
    }

    fn recursive_remove_near(
        &mut self,
        umi: &BitSet,
        idx: usize,
        k: i32,
        max_freq: i32,
        curr: &mut BitSet,
        res: &mut HashSet<Arc<BitSet>>,
        kk: i32,
    ) {
        if k < 0 {
            return;
        }

        if idx >= self.umi_length {
            if self.umi_freq.contains_key(curr)
                && (k == kk || self.umi_freq.get(curr).unwrap() <= &max_freq)
            {
                res.insert(Arc::new(curr.clone()));
                self.umi_freq.remove(curr);
            }
            return;
        }

        for &c in crate::utils::read::ENCODING_IDX.keys() {
            if char_equals(umi, idx, c) {
                self.recursive_remove_near(
                    umi,
                    idx + 1,
                    k,
                    max_freq,
                    char_set(curr, idx, c),
                    res,
                    kk,
                );
            } else {
                self.recursive_remove_near(
                    umi,
                    idx + 1,
                    k - 1,
                    max_freq,
                    char_set(curr, idx, c),
                    res,
                    kk,
                );
            }
        }
    }
}

impl DataStruct for Combo {
    #[allow(unused_variables)]
    fn change(&mut self, umi_freq: HashMap<Arc<BitSet>, i32>, umi_length: usize, max_edits: i32) {
        self.umi_freq = umi_freq;
        self.umi_length = umi_length;
    }
    fn remove_near(
        &mut self,
        umi: &BitSet,
        k: i32,
        max_freq: i32,
    ) -> std::collections::HashSet<Arc<BitSet>> {
        let mut res: HashSet<Arc<BitSet>> = HashSet::new();
        self.recursive_remove_near(
            umi,
            0,
            k,
            max_freq,
            &mut BitSet::new_with_len(self.umi_length * crate::utils::read::ENCODING_LENGTH),
            &mut res,
            k,
        );
        res
    }

    fn contains(&self, umi: &BitSet) -> bool {
        self.umi_freq.contains_key(umi)
    }

    fn stats(&self) -> HashMap<String, f32> {
        HashMap::new()
    }
}
