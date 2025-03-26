use std::collections::HashSet;

use screeps::{
    memory, pathfinder::{self, SearchGoal}, Position, RoomName
};

use crate::{constants::general::GeneralResult, memory::game_memory::GameMemory, state::game::GameState};

use super::{portal_router_single, room_pather_single::{self, PathGoal}, route_costs::{self, economy_creep_costs}, PathfindingOpts};

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn try_find_path(origin: &Position, goal: &PathGoal, opts: PathfindingOpts, game_state: &mut GameState, memory: &GameMemory) -> Result<Vec<Position>, GeneralResult> {
    let mut allowed_rooms: HashSet<RoomName> = find_allowed_rooms(origin, goal, &opts, memory);
    let path = room_pather_single::find_path(*origin, goal, allowed_rooms, &opts.room_pathfinder_opts, game_state, memory);

    path
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
fn find_allowed_rooms(origin: &Position, goal: &PathGoal, opts: &PathfindingOpts, memory: &GameMemory) -> HashSet<RoomName> {
    let mut allowed_rooms: HashSet<RoomName> = HashSet::new();
    allowed_rooms.insert(origin.room_name());

    let goal_room_name = goal.pos.room_name();
    if goal_room_name == origin.room_name() {
        return allowed_rooms;
    }
    
    let Ok(route) = portal_router_single::find_route(origin.room_name(), &goal_room_name, opts, memory) else {
        return allowed_rooms
    };
    
    allowed_rooms.extend(route);
    allowed_rooms
}