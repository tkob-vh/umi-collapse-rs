//! The utils.
//!

pub mod bitset;
pub mod cluster_tracker;
pub mod read;
pub mod read_freq;
pub mod umi_freq;

#[allow(dead_code)]
pub static HASH_CONST: i64 = 31;

use std::hash::{DefaultHasher, Hash, Hasher};

use bitset::BitSet;

pub fn calculate_hash<T: Hash>(t: &T) -> i32 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish() as i32
}

/// fast Hamming distance by using pairwise equidistant encodings for each nucleotide
pub fn umi_dist(a: &BitSet, b: &BitSet) -> i32 {
    a.bit_count_xor(b) / read::ENCODING_DIST
}

#[allow(dead_code)]
pub fn char_equals(a: &BitSet, idx: usize, b: i32) -> bool {
    for i in 0..read::ENCODING_LENGTH {
        if a.get(idx * read::ENCODING_LENGTH + i) != ((b & (1 << i)) != 0) {
            return false;
        }
    }
    true
}

pub fn char_set(a: &mut BitSet, idx: usize, b: i32) -> BitSet {
    for i in 0..read::ENCODING_LENGTH {
        a.set(idx * read::ENCODING_LENGTH + i, (b & (1 << i)) != 0);
    }
    a.clone()
}

fn char_set_n_bit(a: &mut BitSet, idx: usize) -> &BitSet {
    for i in 0..read::ENCODING_LENGTH {
        a.set_n_bit(idx * read::ENCODING_LENGTH + i, true);
    }
    a
}

#[allow(dead_code)]
pub fn char_get(a: &BitSet, idx: usize) -> i32 {
    let mut res: i32 = 0;
    for i in 0..read::ENCODING_LENGTH {
        if a.get(idx * read::ENCODING_LENGTH + i) {
            res |= 1 << i;
        }
    }
    res
}

pub fn to_bitset(s: &[u8]) -> BitSet {
    // Calculate the total number of bits needed
    let total_bits = s.len() * read::ENCODING_LENGTH;

    // Create BitSet with the correct length
    let mut res = BitSet::new_with_len(total_bits);

    for (i, c) in s.iter().enumerate() {
        if let Some(&encoding) = read::ENCODING_MAP.get(c) {
            char_set(&mut res, i, encoding);

            if c == &read::UNDETERMINED_CHAR {
                char_set_n_bit(&mut res, i);
            }
        } else {
            panic!("Unknown character in UMI sequence: {}", c);
        }
    }

    res
}

/// Get the unclipped start position (0-based, inclusive) or end position (0-based, inclusive).
///
/// If not reverse, get start position.
/// This is the alignment start adjusted for any clipped bases.
/// For example, if the read has an alignment start of 100 but the first 4 bases were clipped
/// (hard or soft clipped) then this method will return 96.
///
/// Else, get end position.
/// This is the alignment end adjusted for any clipped bases.
/// For example, if the read has an alignment end of 100 but the last 7 bases were clipped
/// (hard or soft clipped) then this method will return 107.
pub fn get_unclipped_pos(record: &rust_htslib::bam::Record) -> i64 {
    let cigar_str = record.cigar();

    if record.is_reverse() {
        cigar_str.end_pos() - 1 + cigar_str.trailing_softclips() + cigar_str.trailing_hardclips()
    } else {
        cigar_str.pos() - cigar_str.leading_softclips() - cigar_str.leading_hardclips()
    }
}
