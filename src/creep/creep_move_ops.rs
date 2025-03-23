use std::collections::{HashMap, HashSet};

use log::error;
use screeps::{Creep, HasPosition, LocalRoomTerrain, Part, Position, RoomName, Terrain, game};

use crate::{
    constants::{
        creep::MoveTargets,
        general::{GeneralError, GeneralResult},
        move_costs::MAX_COST,
    },
    memory::{creep_memory, game_memory::GameMemory},
    pathfinding::{
        pathfinding_services::{PathfindingOpts, try_find_path},
        room_pather::PathGoals,
    },
    room::room_ops::{self, default_move_costs},
    state::game::GameState,
    utils::{
        self,
        general::{GeneralUtils, pos_range},
        pos::{get_adjacent_positions_conditional, is_xy_exit},
    },
};

use super::my_creep_ops;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn create_move_request(
    creep_name: &str,
    goals: &PathGoals,
    opts: PathfindingOpts,
    game_state: &mut GameState,
    memory: &GameMemory,
) {
    let creep = game_state.creeps.get(creep_name).unwrap();

    // If we have fatigue
    let my_creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();
    if my_creep_state.fatigue > 0 {
        return
    }

    // If we already have a move request
    if my_creep_state.move_request.is_some() {
        return
    }

    // If we are at the goal
    let creep_state = game_state.creep_states.get_mut(creep_name).unwrap();
    if let Some(pos) = creep_state.pos {
        if goals.0.contains_key(&pos) { return }
    }

    let creep_memory = memory.creeps.get(creep_name).unwrap();

    let Ok(path) = try_find_path(&creep.inner().pos(), goals, opts, game_state, memory) else {
        return;
    };

    let creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();
    creep_state.move_request = Some(path[1]);

    log::info!(
        "Created move request for creep {} to {}",
        creep_name,
        path[1]
    );
}

fn assign_move_request(creep_name: &str) {}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn assign_move_target_as_pos(
    creep_name: &str,
    game_state: &mut GameState,
    move_targets: &mut MoveTargets,
) {
    let creep = game_state.creeps.get(creep_name).unwrap();
    let pos = creep.inner().pos();

    move_targets.insert(pos, creep_name.to_string());

    let creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();
    creep_state.move_target = Some(pos);
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn assign_move_target(
    creep_name: &str,
    position: Position,
    game_state: &mut GameState,
    move_targets: &mut MoveTargets,
) {
    move_targets.insert(position, creep_name.to_string());

    let creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();
    creep_state.move_target = Some(position);
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn try_run_move_request(
    creep_name: &str,
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    move_targets: &mut MoveTargets,
) {
    let creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();

    let Some(move_request) = creep_state.move_request else {
        return;
    };

    if let Some(move_target) = creep_state.move_target {
        if move_request == move_target {
            return;
        }

        // If we do have a move target, delete it so we can run the move request

        move_targets.remove(&move_request);
        creep_state.move_target = None;
    };

    let cost = run_move_request(
        creep_name,
        room_name,
        game_state,
        memory,
        move_targets,
        &mut HashSet::new(),
        0,
    );
    if cost < 0 {
        return;
    }

    assign_move_target_as_pos(creep_name, game_state, move_targets)
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
// Use a sparse cost matrix to optimize cost calculations
fn run_move_request(
    creep_name: &str,
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    move_targets: &mut MoveTargets,
    visited_creeps: &mut HashSet<String>,
    cost: i32,
) -> i32 {
    visited_creeps.insert(creep_name.to_string());

    let creep_memory = memory.creeps.get(creep_name).unwrap();

    let target_coord = {
        let creep_state = game_state.my_creep_states.get(creep_name).unwrap();
        creep_state.move_request
    };
    let move_options = get_move_options(creep_name, room_name, game_state, memory, target_coord);

    // TODO: Many of these conditionals should be moved to get_move_options so they can be cached
    for pos in move_options {
        let creep_in_way_name = move_targets.get(&pos);

        if let Some(creep_in_way_name) = creep_in_way_name {
            if visited_creeps.contains(creep_in_way_name) {
                continue;
            }

            // Could be a power creep
            let creep_in_way = game_state.creeps.get(creep_in_way_name);

            if let Some(creep_in_way) = creep_in_way {
                if creep_in_way.inner().get_active_bodyparts(Part::Move) == 0 {
                    continue;
                }
            }
        }
     
        let creep_state = game_state.my_creep_states.get(creep_name).unwrap();

        // Don't allow exits unless we are actively trying to move onto one
        match creep_state.move_request {
            Some(move_request) => {
                if move_request != pos && pos.is_room_edge() {
                    continue;
                }
            }
            None => {
                if pos.is_room_edge() {
                    continue;
                }
            }
        }
    
        let mut potential_cost = cost;
        {
            let creep_state = game_state.my_creep_states.get(creep_name).unwrap();

            if let Some(move_request) = creep_state.move_request {
                if move_request == pos {
                    potential_cost -= 1;
                }
            }
        }
     
        if let Some(creep_in_way_name) = creep_in_way_name {
            // Could be a power creep
            let creep_in_way = game_state.creeps.get(creep_in_way_name);

            if let Some(creep_in_way) = creep_in_way {
                let creep_in_way_state = game_state.my_creep_states.get(creep_in_way_name).unwrap();

                if creep_in_way_state.move_request == Some(pos) {
                    potential_cost += 1;
                }

                let creep_in_way_cost = run_move_request(
                    creep_name,
                    room_name,
                    game_state,
                    memory,
                    move_targets,
                    visited_creeps,
                    potential_cost,
                );
   
                if creep_in_way_cost >= 0 {
                    continue;
                }

                assign_move_target(creep_name, pos, game_state, move_targets);
                return creep_in_way_cost;
            }
        }

        if potential_cost < 0 {
            assign_move_target(creep_name, pos, game_state, move_targets);
        }
        return potential_cost;
    }

    i32::MAX
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn get_move_options(
    creep_name: &str,
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    target_coord: Option<Position>,
) -> Vec<Position> {
    let creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();

    if let Some(move_options) = &creep_state.move_options {
        return move_options.clone();
    };

    let mut move_options: Vec<Position> = Vec::new();

    let creep = game_state.creeps.get(creep_name).unwrap();
    // Would be nice to cache move options on creep state
    // let creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();

    if creep.inner().fatigue() > 0 {
        return move_options;
    }

    let creep_pos = creep.inner().pos();

    if let Some(target_coord) = target_coord {
        if creep_pos == target_coord {
            return move_options;
        };

        move_options.insert(0, target_coord);
        return move_options;
    }

    // Add adjacent positions that are not exits or market to avoid

    let move_costs = default_move_costs(room_name, game_state, memory);

    move_options.extend(get_adjacent_positions_conditional(&creep_pos, &|pos| {
        move_costs.get(pos.xy()) != MAX_COST && !is_xy_exit(pos.x().0, pos.y().0)
    }));

    // Sort by range to an action pos if there is one. Otherwise, shuffle randomly

    let creep_state = game_state.my_creep_states.get(creep_name).unwrap();

    if let Some(action_pos) = creep_state.action_pos {
        move_options.sort_by(|a, b| {
            pos_range(a, &action_pos)
                .partial_cmp(&pos_range(b, &action_pos))
                .unwrap()
        })
    } else {
        fastrand::shuffle(&mut move_options)
    }

    move_options
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn try_run_move_target(creep_name: &str, game_state: &GameState) {
    let creep_state = game_state.my_creep_states.get(creep_name).unwrap();
    let Some(move_target) = creep_state.move_target else {
        return;
    };

    let creep = game_state.creeps.get(creep_name).unwrap();
    let creep_pos = creep.inner().pos();

    if creep_pos == move_target {
        return;
    }

    let Some(direction) = creep_pos.get_direction_to(move_target) else {
        error!("Failed to get direction to move target");
        return;
    };
    creep.inner().move_direction(direction);
}
