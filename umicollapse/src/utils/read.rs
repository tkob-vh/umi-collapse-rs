//! We use `UcRead` and `UcSAMRead` to distinguish them from the structs in rust-htslib.
//! Uc stands for umi-collapse.

use std::{collections::HashMap, fmt::Debug};

use lazy_static::lazy_static;
use pcre2::bytes::{Regex, RegexBuilder};
use tracing::info;

use crate::utils;

pub const ENCODING_DIST: i32 = 2;
pub const ENCODING_LENGTH: usize = 3;
#[allow(dead_code)]
pub const ALPHABET: [char; 5] = ['A', 'T', 'C', 'G', 'N'];
pub const UNDETERMINED: i32 = 0b100;
pub const UNDETERMINED_CHAR: char = 'N';
#[allow(dead_code)]
pub const ANY: i32 = 0b111;

lazy_static! {
    pub static ref ENCODING_MAP: HashMap<char, i32> = {
        let mut m = HashMap::new();
        m.insert('A', 0b000);
        m.insert('T', 0b101);
        m.insert('C', 0b110);
        m.insert('G', 0b011);
        m.insert(UNDETERMINED_CHAR, UNDETERMINED);
        m
    };
    pub static ref ENCODING_IDX: HashMap<i32, i32> = {
        let mut m = HashMap::new();
        m.insert(0b000, 0);
        m.insert(0b101, 1);
        m.insert(0b110, 2);
        m.insert(0b011, 3);
        m.insert(UNDETERMINED, 4);
        m
    };
}

pub trait UcRead: Debug {
    fn get_avg_qual(&self) -> i32;
    fn get_umi(&self, pattern: &Regex) -> utils::bitset::BitSet;
    fn get_umi_length(&self, pattern: &Regex) -> usize;
}

#[derive(Debug, Clone)]
pub struct UcSAMRead {
    record: rust_htslib::bam::Record,
    avg_qual: i32,
}

impl UcSAMRead {
    pub fn new(record: rust_htslib::bam::Record) -> Self {
        let avg: f32 = record.qual().iter().map(|&b| b as f32).sum();

        Self {
            avg_qual: (avg / record.seq_len() as f32) as i32,
            record,
        }
    }

    pub fn umi_pattern(sep: &str) -> Regex {
        // Add debug logging
        let pattern = format!(r"^(?:.*?){}([ATCGN]+)(?:.*?)$", sep);
        info!("UMI pattern: {}", pattern);

        RegexBuilder::new()
            .caseless(true)
            .multi_line(false)
            .jit(true)
            .build(&pattern)
            .expect("Failed to build UMI pattern regex")
    }

    pub fn get_map_qual(&self) -> i32 {
        self.record.mapq() as i32
    }

    pub fn to_sam_record(&self) -> &rust_htslib::bam::Record {
        &self.record
    }
}

impl UcRead for UcSAMRead {
    fn get_umi_length(&self, pattern: &Regex) -> usize {
        let read_name = self.record.qname();
        let caps = pattern.captures(read_name).unwrap().unwrap();
        caps.get(1)
            .expect("No UMI group found in pattern match")
            .as_bytes()
            .len()
    }

    fn get_umi(&self, pattern: &Regex) -> utils::bitset::BitSet {
        let read_name = self.record.qname();

        let caps = pattern
            .captures(read_name)
            .expect("No UMI pattern match found in read name");

        let umi = caps
            .unwrap()
            .get(1)
            .expect("No UMI group found in pattern match")
            .as_bytes();

        if umi.is_empty() {
            panic!("Empty UMI sequence extracted");
        }
        utils::to_bitset(std::str::from_utf8(umi).unwrap())
    }

    fn get_avg_qual(&self) -> i32 {
        self.avg_qual
    }
}

impl PartialEq for UcSAMRead {
    fn eq(&self, other: &Self) -> bool {
        self.record.eq(&other.record)
    }
}
