use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::utils;

use super::bitset::BitSet;

#[allow(dead_code)]
pub struct ClusterStats {
    umi: Arc<BitSet>,
    freq: i32,
    read: Arc<dyn utils::read::UcRead>,
}

impl ClusterStats {
    pub fn new(umi: Arc<BitSet>, freq: i32, read: Arc<dyn utils::read::UcRead>) -> Self {
        Self { umi, freq, read }
    }

    #[allow(dead_code)]
    pub fn get_umi(&self) -> &BitSet {
        &self.umi
    }

    #[allow(dead_code)]
    pub fn get_freq(&self) -> &i32 {
        &self.freq
    }

    #[allow(dead_code)]
    pub fn get_read(&self) -> &Arc<dyn utils::read::UcRead> {
        &self.read
    }
}

pub struct ClusterTracker {
    track: bool,
    offset: usize,
    temp: Vec<Arc<BitSet>>,
    temp_freq: i32,
    to_unique_idx: HashMap<Arc<BitSet>, usize>,
    clusters: Vec<ClusterStats>,
    idx: usize,
}

impl ClusterTracker {
    pub fn new(track: bool) -> Self {
        Self {
            track,
            offset: 0,
            temp: Vec::new(),
            temp_freq: 0,
            to_unique_idx: HashMap::new(),
            clusters: Vec::new(),
            idx: 0,
        }
    }

    #[allow(dead_code)]
    pub fn should_track(&self) -> bool {
        self.track
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    #[allow(dead_code)]
    pub fn get_offset(&self) -> &usize {
        &self.offset
    }

    pub fn add_all(
        &mut self,
        s: &HashSet<Arc<BitSet>>,
        reads: &HashMap<Arc<BitSet>, Arc<utils::read_freq::ReadFreq>>,
    ) {
        if self.track {
            self.temp.extend(s.iter().cloned());

            for umi in s {
                self.temp_freq += reads.get(umi).unwrap().freq;
            }
        }
    }

    pub fn track(&mut self, unique: Arc<BitSet>, read: &Arc<dyn utils::read::UcRead>) {
        if self.track {
            for s in &self.temp {
                self.to_unique_idx.insert(s.to_owned(), self.idx);
            }

            self.clusters.push(ClusterStats::new(
                unique.clone(),
                self.temp_freq,
                read.clone(),
            ));

            self.temp.clear();
            self.temp_freq = 0;
            self.idx += 1;
        }
    }

    #[allow(dead_code)]
    pub fn get_id(&self, umi: &BitSet) -> i32 {
        self.to_unique_idx.get(umi).unwrap().to_owned() as i32
    }

    #[allow(dead_code)]
    pub fn get_stats(&self, id: usize) -> &ClusterStats {
        self.clusters.get(id).unwrap()
    }
}
