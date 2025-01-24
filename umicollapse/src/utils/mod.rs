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

pub fn char_set(a: &mut BitSet, idx: usize, b: i32) -> &mut BitSet {
    for i in 0..read::ENCODING_LENGTH {
        a.set(idx * read::ENCODING_LENGTH + i, (b & (1 << i)) != 0);
    }
    a
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

pub fn to_bitset(s: &str) -> BitSet {
    // Calculate the total number of bits needed
    let total_bits = s.len() * read::ENCODING_LENGTH;

    // Create BitSet with the correct length
    let mut res = BitSet::new_with_len(total_bits);

    for (i, c) in s.chars().enumerate() {
        if let Some(&encoding) = read::ENCODING_MAP.get(&c) {
            char_set(&mut res, i, encoding);

            if c == read::UNDETERMINED_CHAR {
                char_set_n_bit(&mut res, i);
            }
        } else {
            panic!("Unknown character in UMI sequence: {}", c);
        }
    }
    res
}
