#[derive(Debug)]
pub struct TickIntervals {
    pub construction_sites_update: u32,
}

impl TickIntervals {
    pub fn new() -> Self {
        Self {
            construction_sites_update: fastrand::u32(50..100),
        }
    }
}