use std::collections::{HashMap, HashSet};

use screeps::{game, RoomName};

use crate::{constants::general::GeneralResult, memory::game_memory::GameMemory};

use super::pathfinding_services::{PathfindingOpts, RouteCallback};

pub fn find_route(
    origin: RoomName,
    goals: HashSet<RoomName>,
    opts: &PathfindingOpts,
    memory: &GameMemory,
) -> Result<HashSet<RoomName>, GeneralResult> {
    let mut current_generation = HashSet::new();
    current_generation.insert(origin);

    let mut visited = HashSet::new();
    let mut path_from = HashMap::new();
    let mut lowest_next_gen_cost: u32 = u32::MAX;

    while !current_generation.is_empty() {
        let mut next_generation = HashSet::new();
        let lowest_gen_cost = lowest_next_gen_cost;
        lowest_next_gen_cost = u32::MAX;

        for room_name in &current_generation {
            let range_cost = find_range_cost(room_name, &goals, &path_from);
            // If the tile's cost is too high for this generation's threshold, add it to the next generation
            if range_cost > lowest_gen_cost {
                next_generation.insert(*room_name);
                continue;
            }

            let exits = game::map::describe_exits(*room_name).values();
            for adj_room_name in exits {
                // No point in revisiting already explored tiles
                if visited.contains(&adj_room_name) {
                    continue;
                }
                visited.insert(adj_room_name);

                let adj_traverse_cost = (opts.route_callback)(&adj_room_name, memory);
                // If the tile is marked as impassible, skip it
                if adj_traverse_cost == u8::MAX {
                    continue;
                }

                // If we reached a goal, terminate and return the found path
                if goals.contains(&adj_room_name) {
                    let path = find_path(&adj_room_name, &path_from);
                    return Ok(path);
                }

                // We found a valid room to propagate the path to

                next_generation.insert(adj_room_name);
                path_from.insert(adj_room_name, *room_name);

                // Find the lowest cost for the next generation's considerations

                let adj_range_cost =
                    find_range_cost(&adj_room_name, &goals, &path_from);
                let adj_cost = adj_range_cost + adj_traverse_cost as u32;

                if adj_cost < lowest_next_gen_cost {
                    lowest_next_gen_cost = adj_cost
                }
            }
        }

        current_generation = next_generation;
    }

    Err(GeneralResult::Fail)
}

/// navigate backwards accross our map of where tiles came from to construct a path
fn find_path(room_name: &RoomName, path_from: &HashMap<RoomName, RoomName>) -> HashSet<RoomName> {
    let mut path = HashSet::new();
    path.insert(*room_name);

    let mut next_room_name = path_from.get(room_name);

    while next_room_name.is_some() {
        path.insert(*next_room_name.unwrap());
        next_room_name = path_from.get(next_room_name.unwrap());
    }

    path
}

/// Provides a semi-accurate range cost that serves to limit the search area of the algorithm
fn find_range_cost(
    room_name: &RoomName,
    goals: &HashSet<RoomName>,
    path_from: &HashMap<RoomName, RoomName>,
) -> u32 {
    // A rough range
    let goal_cost = find_lowest_cost_goal(room_name, goals);
    // An exact path distance back to the origin
    let origin_cost = find_path(room_name, path_from).len() as u32;

    origin_cost + goal_cost
}

/// Find cost as the lowest linear distance to any goal
fn find_lowest_cost_goal(room_name: &RoomName, goals: &HashSet<RoomName>) -> u32 {
    let mut lowest_cost = f32::INFINITY as u32;
    for goal in goals {
        let cost = game::map::get_room_linear_distance(*room_name, *goal, false);
        if cost >= lowest_cost {
            continue;
        }

        lowest_cost = cost;
    }

    lowest_cost
}
