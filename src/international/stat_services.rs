use screeps::{control, game, RoomName};

use crate::{memory::game_memory::GameMemory, state::game::GameState};

use super::stat_ops::StatOps;

pub struct StatServices;

// functions for statistics to track 
impl StatServices {
    pub fn init_stats(game_state: &GameState, memory: &mut GameMemory) {
        memory.stats.gcl_progress = game::gcl::progress() as u64;
        memory.stats.gcl_total = game::gcl::progress_total() as u64;
        memory.stats.gpl_progress = game::gpl::progress() as u64;
        memory.stats.gpl_total = game::gpl::progress_total() as u64;
        memory.stats.total_creeps = game_state.creeps.len() as u32;
        memory.stats.combined_rcl = StatOps::find_combined_rcl(game_state)   
    }

    fn init_rooms_stats(memory: &mut GameMemory) {

    }

    fn init_room_stats(memory: &mut GameMemory) {

    }

    pub fn update_stats(memory: &mut GameMemory) {

    }

    fn update_rooms_stats(memory: &mut GameMemory) {

    }

    fn update_room_stats(room_name: RoomName, memory: &mut GameMemory) {

    }

    fn update_global_stats(memory: &mut GameMemory) {

    }
}