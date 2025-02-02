//! `Naive` data structure.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::utils::{bitset::BitSet, umi_dist};

use super::DataStruct;

pub struct Naive {
    umi_freq: HashMap<Rc<BitSet>, i32>,
}

impl Naive {
    pub fn new() -> Self {
        Self {
            umi_freq: HashMap::new(),
        }
    }
}

impl DataStruct for Naive {
    #[allow(unused_variables)]
    fn change(&mut self, umi_freq: HashMap<Rc<BitSet>, i32>, umi_length: usize, max_edits: i32) {
        self.umi_freq = umi_freq;
    }

    fn remove_near(&mut self, umi: &BitSet, k: i32, max_freq: i32) -> HashSet<Rc<BitSet>> {
        let mut res: HashSet<Rc<BitSet>> = HashSet::new();
        let umi_freq = &mut self.umi_freq;
        umi_freq.retain(|o, &mut f| {
            let dist: i32 = umi_dist(umi, o);
            if dist <= k && (dist == 0 || f <= max_freq) {
                res.insert(o.clone());
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
