//! We use `UcRead` and `UcSAMRead` to distinguish them from the structs in rust-htslib.
//! Uc stands for umi-collapse.

use std::sync::Arc;
use std::{collections::HashMap, fmt::Debug};

use downcast_rs::{impl_downcast, Downcast};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

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

pub trait UcRead: Downcast + Debug {
    fn get_avg_qual(&self) -> i32;
    fn get_umi(&self, pattern: &Regex) -> utils::bitset::BitSet;
    fn get_umi_length(&self, pattern: &Regex) -> usize;
}
impl_downcast!(UcRead);

#[derive(Debug)]
pub struct UcSAMRead {
    record: Arc<rust_htslib::bam::Record>,
    avg_qual: i32,
}

impl UcSAMRead {
    pub fn new(record: Arc<rust_htslib::bam::Record>) -> Self {
        let avg: f32 = record.qual().iter().map(|&b| b as f32).sum();

        Self {
            avg_qual: (avg / record.seq_len() as f32) as i32,
            record,
        }
    }

    pub fn umi_pattern(sep: &str) -> Regex {
        // Add debug logging
        let pattern = format!(r"^(.*){}([ATCGN]+)(.*?)$", sep);
        println!("UMI pattern: {}", pattern);

        RegexBuilder::new(&pattern)
            .case_insensitive(true)
            .build()
            .expect("Failed to build UMI pattern regex")
    }

    pub fn get_map_qual(&self) -> i32 {
        self.record.mapq() as i32
    }

    pub fn to_sam_record(&self) -> Arc<rust_htslib::bam::Record> {
        self.record.clone()
    }
}

impl UcRead for UcSAMRead {
    fn get_umi_length(&self, pattern: &Regex) -> usize {
        let read_name = String::from_utf8(self.record.qname().to_vec()).unwrap();
        let caps = pattern.captures(&read_name).unwrap();
        caps.get(2).unwrap().as_str().len()
    }

    fn get_umi(&self, pattern: &Regex) -> utils::bitset::BitSet {
        let read_name =
            String::from_utf8(self.record.qname().to_vec()).expect("Invalid UTF-8 in read name");

        let caps = pattern
            .captures(&read_name)
            .expect("No UMI pattern match found in read name");

        let umi = caps
            .get(2)
            .expect("No UMI group found in pattern match")
            .as_str()
            .to_uppercase();

        if umi.is_empty() {
            panic!("Empty UMI sequence extracted");
        }
        utils::to_bitset(&umi)
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
