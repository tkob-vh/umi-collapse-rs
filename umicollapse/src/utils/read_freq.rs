use super::read::UcRead;

#[derive(Debug, Clone)]
pub struct ReadFreq<R: UcRead> {
    pub read: R,
    pub freq: i32,
}

impl<R: UcRead> ReadFreq<R> {
    pub fn new(read: R, freq: i32) -> Self {
        Self { read, freq }
    }
}
