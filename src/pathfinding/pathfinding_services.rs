use std::collections::HashSet;

use screeps::{
    memory, pathfinder::{self, SearchGoal}, Position, RoomName
};

use crate::{constants::general::GeneralResult, memory::game_memory::GameMemory};

use super::{portal_router, room_pather::{self, PathGoals, RoomPathfinderOpts}, route_costs::{self, economy_creep_costs}};

pub struct PathfindingOpts {
    pub room_pathfinder_opts: RoomPathfinderOpts,
    pub route_callback: RouteCallback,
    pub avoid_enemy_creeps: Option<bool>,
}

impl PathfindingOpts {
    pub fn new() -> Self {
        Self {
            room_pathfinder_opts: RoomPathfinderOpts::new(),
            route_callback: economy_creep_costs,
            avoid_enemy_creeps: None,
        }
    }
}

pub type RoomCallback = fn(&RoomName) -> u8;
pub type RouteCallback = fn(&RoomName, &GameMemory) -> u8;

pub fn try_find_path(origin: &Position, goals: &PathGoals, opts: PathfindingOpts, memory: &GameMemory) -> Result<Vec<Position>, GeneralResult> {
    let mut allowed_rooms: HashSet<RoomName> = find_allowed_rooms(origin, goals, &opts, memory);
    let path = room_pather::find_path(*origin, goals, allowed_rooms, &opts.room_pathfinder_opts);

    path
}

fn find_allowed_rooms(origin: &Position, goals: &PathGoals, opts: &PathfindingOpts, memory: &GameMemory) -> HashSet<RoomName> {
    let mut allowed_rooms: HashSet<RoomName> = HashSet::new();
    allowed_rooms.insert(origin.room_name());

    let goal_room_names = HashSet::from_iter(goals.0.iter().map(|pos| {
        pos.0.room_name()
    }));
    let Ok(route) = portal_router::find_route(origin.room_name(), goal_room_names, opts, memory) else {
        return allowed_rooms
    };

    allowed_rooms.extend(route);
    allowed_rooms
}