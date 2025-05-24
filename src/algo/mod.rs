#![allow(clippy::mutable_key_type)]

pub mod adjacency;
pub mod directional;

use std::collections::HashMap;

use crate::{
    data::DataStruct,
    utils::{bitset::BitSet, cluster_tracker::ClusterTracker, read::UcRead, read_freq::ReadFreq},
};

pub trait Algorithm {
    fn apply<'align, R: UcRead, D: DataStruct<'align>>(
        &mut self,
        reads: &'align HashMap<&'align BitSet, &'align ReadFreq<R>>,
        tracker: &mut ClusterTracker<'align, 'align, R>,
        umi_length: usize,
    ) -> Vec<&'align R>;
}
