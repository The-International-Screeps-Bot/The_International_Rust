use super::{simple_allies::SimpleAlliesSegment, stats_segment::StatsSegment};

#[derive(Debug)]
pub struct Segments {
    pub allies: SimpleAlliesSegment,
    pub stats: StatsSegment,
}

impl Segments {
    pub fn new() -> Self {
        Self {
            allies: SimpleAlliesSegment::new(),
            stats: StatsSegment::new(),
        }
    }
}