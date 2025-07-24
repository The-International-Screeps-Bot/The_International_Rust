use std::collections::HashSet;

use screeps::{
    memory, pathfinder::{self, SearchGoal}, Position, RoomName
};

use crate::{constants::general::GeneralResult, memory::game_memory::GameMemory, state::game::GameState};

use super::{portal_router_multi, portal_router_single, room_pather_multi::{self, PathGoals}, route_costs::{self, economy_creep_costs}, PathfindingOpts};

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn try_find_path(origin: &Position, goals: &PathGoals, opts: PathfindingOpts, game_state: &mut GameState, memory: &GameMemory) -> Result<Vec<Position>, GeneralResult> {
    let mut allowed_rooms: HashSet<RoomName> = find_allowed_rooms(origin, goals, &opts, memory);
    let path = room_pather_multi::find_path(*origin, goals, allowed_rooms, &opts.room_pathfinder_opts, game_state, memory);

    path
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
fn find_allowed_rooms(origin: &Position, goals: &PathGoals, opts: &PathfindingOpts, memory: &GameMemory) -> HashSet<RoomName> {
    let origin_room_name = origin.room_name();
    
    let mut allowed_rooms: HashSet<RoomName> = HashSet::new();
    allowed_rooms.insert(origin_room_name);

    let goal_room_names = HashSet::from_iter(goals.0.iter().map(|pos| {
        pos.0.room_name()
    }));
    // Early return if all the goals are in the origin room
    // Comes at the cost of making creeps unable to move around intra-room obstacles
    if goal_room_names.len() == 1 && goal_room_names.contains(&origin_room_name) {
        return allowed_rooms;
    }
    
    let Ok(route) = portal_router_multi::find_route(origin_room_name, goal_room_names, opts, memory) else {
        return allowed_rooms
    };
    
    allowed_rooms.extend(route);
    allowed_rooms
}