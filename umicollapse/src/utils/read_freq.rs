use super::read::UcRead;

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ReadFreq<R: UcRead> {
    pub read: Rc<R>,
    pub freq: i32,
}

impl<R: UcRead> ReadFreq<R> {
    pub fn new(read: Rc<R>, freq: i32) -> Self {
        Self { read, freq }
    }
}
