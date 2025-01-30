//! The merge algorithms.
//!

use std::rc::Rc;

use downcast_rs::{impl_downcast, Downcast};

pub trait Merge: Downcast {
    fn merge(
        &self,
        a: Rc<dyn crate::utils::read::UcRead>,
        b: Rc<dyn crate::utils::read::UcRead>,
    ) -> Rc<dyn crate::utils::read::UcRead>;
}
impl_downcast!(Merge);

pub struct AnyMerge;

impl AnyMerge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Merge for AnyMerge {
    #[allow(unused_variables)]
    fn merge(
        &self,
        a: Rc<dyn crate::utils::read::UcRead>,
        b: Rc<dyn crate::utils::read::UcRead>,
    ) -> Rc<dyn crate::utils::read::UcRead> {
        a
    }
}

pub struct AvgQualMerge;

impl AvgQualMerge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Merge for AvgQualMerge {
    fn merge(
        &self,
        a: Rc<dyn crate::utils::read::UcRead>,
        b: Rc<dyn crate::utils::read::UcRead>,
    ) -> Rc<dyn crate::utils::read::UcRead> {
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

impl Merge for MapQualMerge {
    fn merge(
        &self,
        a: Rc<dyn crate::utils::read::UcRead>,
        b: Rc<dyn crate::utils::read::UcRead>,
    ) -> Rc<dyn crate::utils::read::UcRead> {
        let sam_a: &crate::utils::read::UcSAMRead = a.as_any().downcast_ref().unwrap();
        let sam_b: &crate::utils::read::UcSAMRead = b.as_any().downcast_ref().unwrap();

        if sam_a.get_map_qual() >= sam_b.get_map_qual() {
            a
        } else {
            b
        }
    }
}
