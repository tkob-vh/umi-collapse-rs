//! The merge algorithms.
//!

use std::rc::Rc;

use crate::utils::read::UcRead;

pub trait Merge<R: UcRead> {
    fn merge(&self, a: Rc<R>, b: Rc<R>) -> Rc<R>;
}

pub struct AnyMerge;

impl AnyMerge {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: UcRead> Merge<R> for AnyMerge {
    #[allow(unused_variables)]
    fn merge(&self, a: Rc<R>, b: Rc<R>) -> Rc<R> {
        a
    }
}

pub struct AvgQualMerge;

impl AvgQualMerge {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: UcRead> Merge<R> for AvgQualMerge {
    fn merge(&self, a: Rc<R>, b: Rc<R>) -> Rc<R> {
        if a.get_avg_qual() >= b.get_avg_qual() {
            a
        } else {
            b
        }
    }
}

pub struct MapQualMerge;

impl MapQualMerge {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: UcRead> Merge<R> for MapQualMerge {
    fn merge(&self, a: Rc<R>, b: Rc<R>) -> Rc<R> {
        let sam_a: &crate::utils::read::UcSAMRead = a.as_any().downcast_ref().unwrap();
        let sam_b: &crate::utils::read::UcSAMRead = b.as_any().downcast_ref().unwrap();

        if sam_a.get_map_qual() >= sam_b.get_map_qual() {
            a
        } else {
            b
        }
    }
}
