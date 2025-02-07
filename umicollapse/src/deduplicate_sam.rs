//! The core logic.

#![allow(clippy::mutable_key_type)]

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use ::memchr::arch::x86_64::avx2::memchr::One;
use bumpalo::Bump;
use rust_htslib::bam::{Format, Header, HeaderView, Read, Reader, Record};
use tracing::{debug, info};

use crate::algo::Algorithm;
use crate::cli::Cli;
use crate::data::naive::Naive;
use crate::merge::Merge;
use crate::utils::cluster_tracker::ClusterTracker;
use crate::utils::get_unclipped_pos;
use crate::utils::{
    bitset::BitSet,
    calculate_hash,
    read::{UcRead, UcSAMRead},
    read_freq::ReadFreq,
};

pub trait DeduplicateInterface {
    fn deduplicate_and_merge(&mut self, args: &Cli, start_time: &SystemTime);
}

pub struct DeduplicateSAM<A: Algorithm, M: Merge<UcSAMRead>> {
    algo: A,
    merge_algo: M,
    total_umi_count: usize,
    max_umi_count: usize,
    deduped_count: usize,
    umi_length: usize,
    total_read_count: i32,
    unmapped: i32,
    unpaired: i32,
    chimeric: i32,
    arena: Bump,
}

impl<A: Algorithm, M: Merge<UcSAMRead>> DeduplicateSAM<A, M> {
    pub fn new(args: &Cli, algo: A, merge_algo: M) -> Self {
        let capacity = (1 << 27)
            * (size_of::<Align>() + size_of::<BitSet>() + size_of::<ReadFreq<UcSAMRead>>());

        debug!(
            "The initial capacity of the arena: {} GB",
            capacity / (1024 * 1024)
        );

        Self {
            algo,
            merge_algo,
            total_umi_count: 0,
            max_umi_count: 0,
            deduped_count: 0,
            umi_length: args.umi_length,
            total_read_count: 0,
            unpaired: 0,
            unmapped: 0,
            chimeric: 0,
            arena: Bump::with_capacity(capacity),
        }
    }
}
impl<A: Algorithm, M: Merge<UcSAMRead>> DeduplicateInterface for DeduplicateSAM<A, M> {
    fn deduplicate_and_merge(&mut self, args: &Cli, start_time: &SystemTime) {
        // Set default umi pattern
        let regex = UcSAMRead::umi_pattern(args.umi_separator);
        let one_search = One::new(args.umi_separator).expect("failed to create a new searcher");

        // Construct the reader.
        let mut reader = Reader::from_path(&args.input).expect("Invalid input path");

        reader
            .set_threads(args.num_threads)
            .expect("Failed to set the number of threads for reader.");

        // Construct the writer.
        let mut writer: UcWriter =
            UcWriter::new(&args.input, &args.output, &reader, args.paired, args);

        let mut align: HashMap<&Align, HashMap<&BitSet, &ReadFreq<UcSAMRead>>> =
            HashMap::with_capacity(1 << 20);

        let mut record = Record::new();

        while let Some(r) = reader.read(&mut record) {
            r.expect("Failed to parse record");

            if args.paired && record.is_paired() && record.is_last_in_template() {
                continue;
            }

            self.total_read_count += 1;

            if record.is_unmapped() {
                self.unmapped += 1;
                if args.keep_unmapped {
                    writer.write(&record).expect("Failed to write the record");
                }
                continue;
            }

            if args.paired {
                if !record.is_paired() {
                    self.unpaired += 1;
                    if args.remove_unpaired {
                        continue;
                    }
                }

                if record.is_paired() && record.is_mate_unmapped() {
                    self.unmapped += 1;
                    continue;
                }

                if record.is_paired() && !record.tid().eq(&record.mtid()) {
                    self.chimeric += 1;
                    if args.remove_chimeric {
                        continue;
                    }
                }
            }

            let unclipped_pos = get_unclipped_pos(&record);

            let alignment: &mut Align = if args.paired {
                self.arena.alloc(Align::Paired(PairedAlignment::new(
                    record.is_reverse(),
                    unclipped_pos,
                    reader.header().tid2name(record.tid() as u32).to_vec(),
                    record.insert_size(),
                )))
            } else {
                self.arena.alloc(Align::Unpaired(Alignment::new(
                    record.is_reverse(),
                    unclipped_pos,
                    reader.header().tid2name(record.tid() as u32).to_vec(),
                )))
            };

            let umi_reads: &mut HashMap<_, _> = align
                .entry(alignment)
                .or_insert_with(|| HashMap::with_capacity(4));

            let read = UcSAMRead::new(record.clone());

            if self.umi_length == 0 {
                self.umi_length = read.get_umi_length(&regex);
            }

            let umi = self.arena.alloc(read.get_umi(&one_search, self.umi_length));

            match umi_reads.entry(umi) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(self.arena.alloc(ReadFreq::new(read, 1)));
                }
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    let keep_existing = self.merge_algo.merge(&e.get().read, &read);
                    let new_freq = e.get().freq + 1;
                    if !keep_existing {
                        e.insert(self.arena.alloc(ReadFreq::new(read, new_freq)));
                    } else {
                        let freq_p = *e.get() as *const _ as *mut ReadFreq<UcSAMRead>;
                        unsafe {
                            (*freq_p).freq = new_freq;
                        }
                    }
                }
            }
        }
        let mid_time = SystemTime::now();

        info!(
            "UMI collapsing reading finished in {:?} seconds",
            mid_time.duration_since(*start_time).unwrap().as_secs_f32()
        );

        // debug!("Number of input reads: {}", self.total_read_count);
        // debug!("Number of removed unmapped reads: {}", self.unmapped);
        // if args.paired {
        //     debug!("Number of unpaired reads: {}", self.unpaired);
        //     debug!("Number of chimeric reads: {}", self.chimeric);
        // }
        //
        // debug!("Number of unique alignment positions: {}", align.len());

        drop(reader);

        debug!("Done reading input file into memory!");

        let align_pos_count: usize = align.len();

        let mut cluster_trackers: Option<HashMap<&Align, ClusterTracker<UcSAMRead>>> =
            if args.track_clusters {
                Some(HashMap::new())
            } else {
                None
            };

        for (alignment, umi_reads) in align.iter() {
            let mut curr_trakcer = ClusterTracker::new(args.track_clusters);

            // TODO: fix the Naive
            let dedupped =
                self.algo
                    .apply::<UcSAMRead, Naive>(umi_reads, &mut curr_trakcer, self.umi_length);

            curr_trakcer.set_offset(self.deduped_count);

            self.total_umi_count += umi_reads.len();
            self.max_umi_count = std::cmp::max(self.max_umi_count, umi_reads.len());
            self.deduped_count += dedupped.len();

            if args.track_clusters {
                cluster_trackers
                    .as_mut()
                    .unwrap()
                    .insert(alignment, curr_trakcer);
            } else {
                for read in dedupped {
                    writer
                        .write(read.to_sam_record())
                        .expect("Failed to write the record");
                }
            }
        }

        // second pass to tag reads with their cluster and other stats
        if args.track_clusters {
            debug!("Done with the first pass for tracking clusters");
            //TODO: to be finished.
        }

        writer.close();

        debug!("Number of input reads: {}", self.total_read_count);
        debug!("Number of removed unmapped reads: {}", self.unmapped);
        if args.paired {
            debug!("Number of unpaired reads: {}", self.unpaired);
            debug!("Number of chimeric reads: {}", self.chimeric);
        }

        debug!("Number of unique alignment positions: {}", align_pos_count);
        debug!("Number of UMIs: {}", self.total_umi_count);
        debug!(
            "Average number of UMIs per alignment position: {}",
            self.total_umi_count as f64 / align_pos_count as f64
        );
        debug!(
            "Max number of UMIs over all alignment positions: {}",
            self.max_umi_count
        );

        if args.track_clusters {
            debug!("Number of groups of reads: {}", self.deduped_count);
        } else {
            debug!(
                "Number of reads after deduplicating: {}",
                self.deduped_count
            );
        }
    }
}
const HASH_CONST: i32 = 31;

struct ReverseRead {
    coord: i64,
    ref_str: Vec<u8>,
    name: Vec<u8>,
}

impl ReverseRead {
    fn new(name: Vec<u8>, ref_str: Vec<u8>, coord: i64) -> Self {
        Self {
            name,
            ref_str,
            coord,
        }
    }
}

impl PartialEq for ReverseRead {
    fn eq(&self, other: &Self) -> bool {
        if self.ref_str.len() != other.ref_str.len() || self.name.len() != other.name.len() {
            return false;
        }

        self.ref_str == other.ref_str && self.name == other.name
    }
}

impl Eq for ReverseRead {}

impl Hash for ReverseRead {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash = 0i32;

        // name hash
        for &b in &self.name {
            hash = hash.wrapping_mul(HASH_CONST).wrapping_add(b as i32);
        }

        // ref_str hash
        for &b in &self.ref_str {
            hash = hash.wrapping_mul(HASH_CONST).wrapping_add(b as i32);
        }

        // coord hash
        hash = hash
            .wrapping_mul(HASH_CONST)
            .wrapping_add(self.coord as i32);

        state.write_i32(hash);
    }
}

impl Ord for ReverseRead {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.coord
            .cmp(&other.coord)
            .then_with(|| self.ref_str.cmp(&other.ref_str))
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for ReverseRead {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct UcWriter {
    thread_num: usize,
    paired: bool,
    writer: rust_htslib::bam::Writer,
    in_file: std::path::PathBuf,
    ref_str: Option<Vec<u8>>,
    set: HashSet<ReverseRead>,
    header_view: HeaderView,
}

impl UcWriter {
    pub fn new(
        in_file: &std::path::PathBuf,
        out_file: &std::path::PathBuf,
        r: &Reader,
        paired: bool,
        args: &Cli,
    ) -> Self {
        let mut writer = rust_htslib::bam::Writer::from_path(
            out_file,
            &Header::from_template(r.header()),
            Format::Bam,
        )
        .unwrap();
        writer
            .set_threads(args.num_threads)
            .expect("Failed to set the number of threads for writer");

        Self {
            thread_num: args.num_threads,
            in_file: if paired {
                in_file.to_owned()
            } else {
                std::path::PathBuf::new()
            },
            set: HashSet::new(),
            header_view: r.header().to_owned(),
            writer,
            paired,
            ref_str: None,
        }
    }

    pub fn write(&mut self, record: &Record) -> std::io::Result<()> {
        if self.paired {
            let curr_ref: Vec<u8> = self.header_view.tid2name(record.tid() as u32).to_vec();

            if self.ref_str.is_none() {
                self.ref_str = Some(curr_ref);
            } else if !self.ref_str.as_ref().unwrap().eq(&curr_ref) {
                self.write_reversed(false);
                self.ref_str = Some(curr_ref);
            }

            if record.is_paired() {
                self.set.insert(ReverseRead::new(
                    record.qname().to_vec(),
                    self.header_view.tid2name(record.mtid() as u32).to_vec(),
                    record.mpos(),
                ));
            }
        }

        self.writer
            .write(record)
            .expect("Failed to write the record");

        Ok(())
    }

    pub fn close(&mut self) {
        if self.paired {
            self.write_reversed(true);
        }
    }

    fn write_reversed(&mut self, full_pass: bool) {
        if self.ref_str.is_none() {
            return;
        }

        let mut reader: Reader =
            Reader::from_path(&self.in_file).expect("Failed to open the input file");

        reader
            .set_threads(self.thread_num)
            .expect("Failed to set the number of threads for reader");

        let mut record = Record::new();

        while let Some(r) = reader.read(&mut record) {
            r.expect("Failed to parse record");

            if !record.is_unmapped()
                && record.is_paired()
                && record.is_last_in_template()
                && !record.is_mate_unmapped()
            {
                if !full_pass {
                    let record_ref = self.header_view.tid2name(record.tid() as u32).to_vec();
                    if !self.ref_str.as_ref().unwrap().eq(&record_ref) {
                        continue;
                    }
                }

                let rev_read = ReverseRead::new(
                    record.qname().to_vec(),
                    self.header_view.tid2name(record.tid() as u32).to_vec(),
                    record.pos(),
                );

                if self.set.contains(&rev_read) {
                    // Write reversed read while preserving flags
                    self.writer
                        .write(&record)
                        .expect("Failed to write the record");
                    self.set.remove(&rev_read);
                }
            }
        }
    }
}

#[allow(dead_code)]
struct AlignReads {
    pub latest: i32,
    pub umi_read: Option<HashMap<BitSet, ReadFreq<UcSAMRead>>>,
}

impl AlignReads {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            latest: 0,
            umi_read: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Align {
    Unpaired(Alignment),
    Paired(PairedAlignment),
}

#[derive(Debug)]
struct Alignment {
    strand: bool,
    coord: i64,
    ref_str: Vec<u8>,
}

impl Alignment {
    #[allow(dead_code)]
    fn new(strand: bool, coord: i64, ref_str: Vec<u8>) -> Self {
        Self {
            strand,
            coord,
            ref_str,
        }
    }

    #[allow(dead_code)]
    fn get_ref(&self) -> &[u8] {
        &self.ref_str
    }
}

impl PartialEq for Alignment {
    fn eq(&self, other: &Self) -> bool {
        if self.strand != other.strand || self.coord != other.coord {
            return false;
        }
        self.ref_str == other.ref_str
    }
}

impl Eq for Alignment {}

impl Hash for Alignment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash: i32 = if self.strand { 1231 } else { 1237 };

        hash = hash * HASH_CONST + self.coord as i32;
        hash = hash * HASH_CONST + calculate_hash(&self.ref_str);

        state.write_i32(hash);
    }
}

impl Ord for Alignment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 按照Java版本相同的比较优先级
        self.strand
            .cmp(&other.strand)
            .then_with(|| self.coord.cmp(&other.coord))
            .then_with(|| self.ref_str.cmp(&other.ref_str))
    }
}

impl PartialOrd for Alignment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
struct PairedAlignment {
    strand: bool,
    coord: i64,
    ref_str: Vec<u8>,
    tlen: i64,
}

impl PairedAlignment {
    #[allow(dead_code)]
    fn new(strand: bool, coord: i64, ref_str: Vec<u8>, tlen: i64) -> Self {
        Self {
            strand,
            coord,
            ref_str,
            tlen,
        }
    }
}
