use std::str::FromStr;

use js_sys::JsString;
use log::error;
use screeps::raw_memory;

use crate::{
    constants::{general::GeneralError, segments::ALLIES_SEGMENT},
    memory::game_memory::GameMemory,
    state::{game::GameState, simple_allies::{SegmentData, SimpleAlliesSegment}},
};

pub fn read_ally_segment(game_state: &mut GameState, memory: &mut GameMemory) {
    if memory.allies.is_empty() {
        error!("Failed to find an ally for simpleAllies, you probably have none :(");
        return;
    };

    let allies_list: Vec<&String> = memory.allies.keys().collect();
    let current_ally_name = allies_list[(game_state.tick + 1) as usize % allies_list.len()];
    let next_ally_name = allies_list[game_state.tick as usize % allies_list.len()];

    raw_memory::set_active_foreign_segment(
        &JsString::from_str(next_ally_name.as_str()).unwrap(),
        Some(ALLIES_SEGMENT),
    );

    let Some(foreign_segment) = raw_memory::foreign_segment() else {
        return;
    };

    if foreign_segment.username() != current_ally_name {
        return;
    };

    let segment_str = foreign_segment.data().as_string().unwrap();

    let Ok(segment_data) = serde_json::from_str::<SegmentData>(&segment_str) else {
        return;
    };

    game_state.segments.allies.ally_requests = Some(segment_data.requests);
    game_state.segments.allies.ally_econ_info = segment_data.econ_info;
}

pub fn write_ally_segment(game_state: &mut GameState, memory: &mut GameMemory) {

    // Ensure that we don't have 10 or more segments open

    raw_memory::segments().set(ALLIES_SEGMENT, serde_json::to_string(&game_state.segments.allies.my_segment_data).unwrap());
    raw_memory::set_public_segments(&[ALLIES_SEGMENT]);
}