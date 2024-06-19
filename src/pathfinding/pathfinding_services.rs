use std::collections::HashSet;

use screeps::{
    pathfinder::{self, SearchGoal},
    Position, RoomName,
};

use crate::constants::general::GeneralResult;

use super::{pathfinder_ops::{self, PathGoals}, portal_router};

pub struct PathfindingOpts {
    pub room_callback: RoomCallback,
    pub route_callback: RouteCallback,
    pub avoid_enemy_creeps: bool,
}

pub type RoomCallback = fn(&RoomName) -> u8;
pub type RouteCallback = fn(&RoomName) -> u8;

pub fn try_find_path(origin: &Position, goals: &PathGoals, opts: PathfindingOpts) -> Result<Vec<Position>, GeneralResult> {
    let mut allowed_rooms = find_allowed_rooms(origin, goals, opts.route_callback);

    let path = generate_path(origin, allowed_rooms, goals);
    path
}

fn find_allowed_rooms(origin: &Position, goals: &PathGoals, rouote_callback: RouteCallback) -> HashSet<RoomName> {
    let mut allowed_rooms: HashSet<RoomName> = HashSet::new();
    allowed_rooms.insert(origin.room_name());

    let goal_room_names = HashSet::from_iter(goals.iter().map(|pos| {
        pos.0.room_name()
    }));
    let Ok(route) = portal_router::find_route(origin.room_name(), goal_room_names, rouote_callback) else {
        return allowed_rooms
    };

    allowed_rooms.extend(route);
    allowed_rooms
}

fn generate_path(origin: &Position, allowed_rooms: HashSet<RoomName>, goals: &PathGoals) -> Result<Vec<Position>, GeneralResult> {

    pathfinder_ops::find_path(*origin, goals, allowed_rooms, None)
}
