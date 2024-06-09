use std::collections::HashSet;

use screeps::{
    pathfinder::{self, SearchGoal},
    Position, RoomName,
};

use crate::constants::general::GeneralResult;

use super::pathfinder_ops::{self, PathGoals};

pub struct PathfindingOpts {
    pub room_callback: fn(&RoomName) -> u8,
    pub route_callback: fn(&RoomName) -> u8,
    pub avoid_enemy_creeps: bool,
}

pub fn try_find_path(origin: &Position, goals: &PathGoals) -> Result<Vec<Position>, GeneralResult> {
    let mut allowed_rooms = find_allowed_rooms(origin, goals);

    let path = generate_path(origin, goals);
    path
}

fn find_allowed_rooms(origin: &Position, goals: &PathGoals) {
    let mut allowed_rooms: HashSet<RoomName> = HashSet::new();
    allowed_rooms.insert(origin.room_name());
}

fn generate_path(origin: &Position, goals: &PathGoals) -> Result<Vec<Position>, GeneralResult> {
    pathfinder_ops::find_path(*origin, goals, None)
}
