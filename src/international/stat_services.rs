use screeps::{control, game, RoomName};

use crate::{memory::game_memory::GameMemory, state::game::GameState};

use super::stat_ops;

// functions for statistics to track
pub fn tick_update(game_state: &mut GameState, memory: &mut GameMemory) {
    {
        let stats = &mut game_state.segments.stats;

        stats.gcl_progress = game::gcl::progress() as u64;
        stats.gcl_total = game::gcl::progress_total() as u64;
        stats.gpl_progress = game::gpl::progress() as u64;
        stats.gpl_total = game::gpl::progress_total() as u64;
        stats.total_creeps = game_state.creeps.len() as u32;
    }

    game_state.segments.stats.combined_rcl = stat_ops::find_combined_rcl(game_state)
}

fn init_rooms_stats(memory: &mut GameMemory) {}

fn init_room_stats(memory: &mut GameMemory) {}

pub fn update_stats(memory: &mut GameMemory) {}

fn update_rooms_stats(memory: &mut GameMemory) {}

fn update_room_stats(room_name: RoomName, memory: &mut GameMemory) {}

fn update_global_stats(memory: &mut GameMemory) {}
