use screeps::game;

pub struct GeneralUtils;

impl GeneralUtils {
    pub fn is_tick_interval(interval: u32) -> bool {
        let tick = game::time();

        tick % interval == 0
    }
}