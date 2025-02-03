#![allow(clippy::mutable_key_type)]

use std::collections::{HashMap, HashSet};

use crate::utils;

use super::bitset::BitSet;
use super::read::UcRead;

#[allow(dead_code)]
pub struct ClusterStats<'cluster_stats, R: UcRead> {
    umi: &'cluster_stats BitSet,
    freq: i32,
    read: &'cluster_stats R,
}

impl<'a, R: UcRead> ClusterStats<'a, R> {
    pub fn new(umi: &'a BitSet, freq: i32, read: &'a R) -> Self {
        Self { umi, freq, read }
    }

    #[allow(dead_code)]
    pub fn get_umi(&self) -> &BitSet {
        self.umi
    }

    #[allow(dead_code)]
    pub fn get_freq(&self) -> &i32 {
        &self.freq
    }

    #[allow(dead_code)]
    pub fn get_read(&self) -> &R {
        self.read
    }
}

pub struct ClusterTracker<'bitset, 'cluster_stats, R: UcRead> {
    track: bool,
    offset: usize,
    temp: Vec<&'bitset BitSet>,
    temp_freq: i32,
    to_unique_idx: HashMap<&'bitset BitSet, usize>,
    clusters: Vec<ClusterStats<'cluster_stats, R>>,
    idx: usize,
}

impl<'a, 'b, R: UcRead> ClusterTracker<'a, 'b, R> {
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
        s: &HashSet<&'a BitSet>,
        reads: &HashMap<BitSet, utils::read_freq::ReadFreq<R>>,
    ) {
        if self.track {
            self.temp.extend(s.iter().cloned());

            for umi in s {
                self.temp_freq += reads.get(umi).unwrap().freq;
            }
        }
    }

    pub fn track(&mut self, unique: &'b BitSet, read: &'b R) {
        if self.track {
            for s in &self.temp {
                self.to_unique_idx.insert(s.to_owned(), self.idx);
            }

            self.clusters
                .push(ClusterStats::new(unique, self.temp_freq, read));

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
    pub fn get_stats(&self, id: usize) -> &ClusterStats<R> {
        self.clusters.get(id).unwrap()
    }
}
