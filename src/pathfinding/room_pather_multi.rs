use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use screeps::{Direction, Position, RoomName, RoomVisual, pathfinder::SearchGoal, visual};
use screeps_utils::sparse_cost_matrix::{self, SparseCostMatrix};

use crate::{
    constants::general::{GeneralResult, DIAGONAL_CARDINAL_DIRECTIONS, DIRECTIONS}, memory::game_memory::GameMemory, room::room_ops::{self, sparse_terrain, terrain}, state::game::GameState, utils::{general::GeneralUtils, pos::get_positions_in_range_in_room, visuals::visualize_path}
};

use super::{room_costs::economy_room_costs, RoomPathfinderOpts};

/// Position -> range map
pub struct PathGoals(pub HashMap<Position, u8>);

impl PathGoals {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn new_from_pos(pos: Position, range: u8) -> Self {
        let mut goals = HashMap::new();
        goals.insert(pos, range);
        Self(goals)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PathfinderOpenSetEntry {
    /// the position the entry is for
    pos: Position,
    /// g_score represents the cost of the best known path to get to this node
    g_score: u32,
    /// f_score represents the estimated total cost of a path through this node,
    /// adding the best known cost to the node (the g_score) to the heuristic's estimate of the
    /// cost to get from the node to the goal
    f_score: u32,
    /// direction this entry was opened from
    open_dir: Option<Direction>,
}

impl Ord for PathfinderOpenSetEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // we're using a max-heap but the behavior we want is min-heap, usual ordering is inverted
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for PathfinderOpenSetEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PathfinderOpenSetEntry {
    pub fn new(
        pos: Position,
        g_score: u32,
        goals_set: &HashSet<Position>,
        open_dir: Option<Direction>,
    ) -> Self {
        let heuristic_cost = get_heuristic_cost_to_closest_goal(pos, goals_set);

        PathfinderOpenSetEntry {
            pos,
            g_score,
            f_score: g_score + heuristic_cost,
            open_dir,
        }
    }
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn find_path(
    origin: Position,
    goals: &PathGoals,
    allowed_rooms: HashSet<RoomName>,
    opts: &RoomPathfinderOpts,
    game_state: &mut GameState,
    memory: &GameMemory,
) -> Result<Vec<Position>, GeneralResult> {
    log::info!("Trying to find a path");
    let origin_room_name = origin.room_name();

    let mut open_set = BinaryHeap::new();
    let mut visited = HashMap::new();

    let goals_exact = goals.0.keys().copied().collect::<HashSet<Position>>();
    let goals_set = {
        let mut goals_set = goals_exact.clone();
        for goal in goals.0.iter() {
            for position in get_positions_in_range_in_room(goal.0, 1) {
                goals_set.insert(position);
            }
        }
        goals_set
    };
    let mut rooms_costs: HashMap<RoomName, SparseCostMatrix> = HashMap::new();

    open_set.push(PathfinderOpenSetEntry::new(origin, 0, &goals_exact, None));
    visited.insert(origin, None);

    while let Some(open_set_entry) = open_set.pop() {
        // Traverse diagonals before cardinals? Might not do what I think it will
        for direction in DIAGONAL_CARDINAL_DIRECTIONS {
            if Some(-direction) == open_set_entry.open_dir {
                continue;
            }
            let Ok(pos) = open_set_entry.pos.checked_add((direction).into()) else {
                continue;
            };

            if visited.contains_key(&pos) {
                continue;
            }
            visited.insert(pos, Some(direction));

            // Need to check if it's sufficiently in range to any of the goals
            if goals_set.contains(&pos) {
                let path = resolve_completed_path(pos, &visited);
                let mut path_vec = path.into_iter().collect::<Vec<Position>>();
                path_vec.reverse();

                visualize_path(&path_vec);

                return Ok(path_vec);
            }

            let room_name = pos.room_name();
            if !opts.allow_outside_origin_room && room_name != origin_room_name {
                continue;
            }

            let room_costs = rooms_costs
                .entry(room_name)
                .or_insert((opts.cost_callback)(&room_name, game_state, memory));

            // let room_costs = match rooms_costs.get(&room_name) {
            //     Some(costs) => costs,
            //     None => &rooms_costs.insert(room_name, (opts.cost_callback)(&room_name, game_state)).unwrap(),
            // };

            let traverse_cost = room_costs.get(pos.xy());
            if traverse_cost == u8::MAX {
                continue;
            }

            open_set.push(PathfinderOpenSetEntry::new(
                pos,
                open_set_entry.g_score + traverse_cost as u32,
                &goals_exact,
                Some(direction),
            ));
        }
    }

    Err(GeneralResult::Fail)
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
/// Find cost as the lowest manhattan distance to any goal
fn get_heuristic_cost_to_closest_goal(pos: Position, goals: &HashSet<Position>) -> u32 {
    let mut lowest_cost = u32::MAX;
    let pos_world_x = pos.world_x();
    let pos_world_y = pos.world_y();

    for goal in goals {
        let cost = pos_world_x.abs_diff(goal.world_x()) + pos_world_y.abs_diff(goal.world_y());
        if cost < lowest_cost {
            lowest_cost = cost;
        }
    }
    lowest_cost
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
/// navigate backwards across our map of where tiles came from to construct a path
fn resolve_completed_path(
    pos: Position,
    visited: &HashMap<Position, Option<Direction>>,
) -> Vec<Position> {
    let mut path = Vec::new();
    path.push(pos);

    let mut cursor_pos = pos;

    while let Some(optional_search_direction) = visited.get(&cursor_pos) {
        match optional_search_direction {
            Some(search_dir) => {
                if let Ok(next_pos) = cursor_pos.checked_add((-*search_dir).into()) {
                    path.push(next_pos);
                    cursor_pos = next_pos;
                }
            }
            None => break,
        }
    }

    path
}
