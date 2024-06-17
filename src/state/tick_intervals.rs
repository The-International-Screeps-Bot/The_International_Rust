#[derive(Debug)]
pub struct TickIntervals {
    pub construction_sites_update: u32,
    pub decay_player_data: u32,
    pub write_stats: u32,
}

impl TickIntervals {
    pub fn new() -> Self {
        Self {
            construction_sites_update: fastrand::u32(50..100),
            decay_player_data: fastrand::u32(300..500),
            write_stats: 5,
        }
    }
}