use super::simple_allies::SimpleAlliesSegment;

#[derive(Debug)]
pub struct Segments {
    pub allies: SimpleAlliesSegment
}

impl Segments {
    pub fn new() -> Self {
        Self {
            allies: SimpleAlliesSegment::new(),
        }
    }
}