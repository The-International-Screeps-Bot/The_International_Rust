#[derive(Debug)]
pub struct TickIntervals {
    pub construction_sites_update: u32,
    pub decay_player_data: u32,
    pub write_stats: u32,
    pub write_memory: u32,
}

impl TickIntervals {
    pub fn new() -> Self {
        let mut rng = fastrand::Rng::new();
        
        Self {
            construction_sites_update: rng.u32(50..100),
            decay_player_data: rng.u32(300..500),
            write_stats: 5,
            write_memory: 1,// rng.u32(10..20),
        }
    }
}