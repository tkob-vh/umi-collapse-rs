use std::cell::Cell;
use std::{cmp::Ordering, hash::Hash};

use crate::utils;

const CHUNK_SIZE: usize = 64;

#[derive(Debug)]
pub struct BitSet {
    bits: Vec<i64>,
    n_bits: Option<Vec<i64>>,
    recalc_hash: Cell<bool>,
    hash: Cell<i32>,
}

impl BitSet {
    pub fn new_with_len(length: usize) -> Self {
        let capacity = length / CHUNK_SIZE + (if length % CHUNK_SIZE == 0 { 0 } else { 1 });

        let bits: Vec<i64> = vec![0; capacity];
        Self {
            bits,
            n_bits: None,
            recalc_hash: Cell::new(true),
            hash: Cell::new(0),
        }
    }

    pub fn new_with_bits(bits: Vec<i64>) -> Self {
        Self {
            bits,
            n_bits: None,
            recalc_hash: Cell::new(true),
            hash: Cell::new(0),
        }
    }

    pub fn new_with_hash(bits: Vec<i64>, hash: i32) -> Self {
        Self {
            bits,
            n_bits: None,
            recalc_hash: Cell::new(false),
            hash: Cell::new(hash),
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        self.bits[idx / CHUNK_SIZE] & (1 << (idx % CHUNK_SIZE)) != 0
    }

    /// does not set the nBits array, so distance calculations could be wrong if not careful!
    pub fn set(&mut self, idx: usize, bit: bool) {
        self.recalc_hash.set(true);
        let i = idx / CHUNK_SIZE;
        let j = idx % CHUNK_SIZE;
        self.bits[i] = if bit {
            self.bits[i] | (1i64 << j)
        } else {
            self.bits[i] & !(1i64 << j)
        };
    }

    pub fn set_n_bit(&mut self, idx: usize, bit: bool) {
        if self.n_bits.is_none() {
            self.n_bits = Some(Vec::with_capacity(self.bits.len()));
            self.n_bits.as_mut().unwrap().resize(self.bits.len(), 0);
        }

        let i: usize = idx / CHUNK_SIZE;
        let j: usize = idx % CHUNK_SIZE;

        let tmp = self.n_bits.as_deref().unwrap()[i];

        self.n_bits.as_mut().unwrap()[i] = if bit { tmp | (1 << j) } else { tmp & !(1 << j) };
    }

    pub fn bit_count_xor(&self, o: &BitSet) -> i32 {
        let mut res: i32 = 0;
        let encoding_length = utils::read::ENCODING_LENGTH as i32;

        for i in 0..self.bits.len() {
            let self_n_bits = self.n_bits.as_ref().map_or(0, |n| n[i]);
            let o_n_bits = o.n_bits.as_ref().map_or(0, |n| n[i]);

            let xor = self_n_bits ^ o_n_bits;
            res += i64::count_ones(xor | (self.bits[i] ^ o.bits[i])) as i32
                - i64::count_ones(xor) as i32 / encoding_length;
        }

        res
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool {
        if self.bits.len() != other.bits.len() {
            return false;
        }
        self.bits.as_slice() == other.bits.as_slice()
    }
}

impl Eq for BitSet {}

impl Ord for BitSet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.bits.len() != other.bits.len() {
            return self.bits.len().cmp(&other.bits.len());
        }
        self.bits.as_slice().cmp(other.bits.as_slice())
    }
}

impl PartialOrd for BitSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Clone for BitSet {
    fn clone(&self) -> Self {
        if self.recalc_hash.get() {
            Self::new_with_bits(self.bits.clone())
        } else {
            Self::new_with_hash(self.bits.clone(), self.hash.get())
        }
    }
}

impl Hash for BitSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.recalc_hash.get() {
            let mut h = 1234i64;

            let mut i = self.bits.len();
            while i > 0 {
                i -= 1;
                h ^= self.bits[i] * (i as i64 + 1);
            }

            self.hash.set((h ^ (h >> 32)) as i32);
            self.recalc_hash.set(false);
        }

        state.write_i32(self.hash.get());
    }
}
