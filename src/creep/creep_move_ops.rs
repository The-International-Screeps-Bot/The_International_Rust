use std::collections::{HashMap, HashSet};

use screeps::{Creep, HasPosition, Position};

use crate::{constants::general::{GeneralError, GeneralResult}, memory::game_memory::GameMemory, state::game::GameState, utils::{self, general::GeneralUtils}};

pub struct CreepMoveOps;

impl CreepMoveOps {
    pub fn create_move_request(creep_name: &String, origin: &Position, game_state: &GameState, memory: &GameMemory) {

    }

    fn assign_move_request(creep_name: &String) {
        
    }

    pub fn try_run_move_request(creep_name: &String, game_state: &GameState, avoid_positions: &mut HashSet<Position>) -> GeneralResult {

        let creep = game_state.creeps.get(creep_name).unwrap();

        let creep_positions: HashMap<Position, String> = HashMap::new();

        let target_pos = creep.inner().pos();

        let move_pos = Self::find_move_coord(creep_name, game_state, avoid_positions, Some(&target_pos));
        let Some(move_pos) = move_pos else {
            return GeneralResult::Fail
        };

        let creep_at_pos_name = creep_positions.get(&move_pos);
        if let Some(creep_at_pos_name) = creep_at_pos_name {
            let creep_at_pos = game_state.creeps.get(creep_at_pos_name);
            if let Some(creep_at_pos) = creep_at_pos {
                avoid_positions.insert(creep.inner().pos());
                avoid_positions.insert(move_pos);

                let move_result = Self::try_run_move_request(creep_name, game_state, avoid_positions);
                if move_result != GeneralResult::Success {
                    return move_result
                }
            }
        }

        Self::run_move_request(creep_name);
        GeneralResult::Success
    }

    fn find_move_coord(creep_name: &String, game_state: &GameState, avoid_positions: &HashSet<Position>, target_pos: Option<&Position>) -> Option<Position> {

        let creep = game_state.creeps.get(creep_name).unwrap();

        let mut move_pos: Option<Position> = None;
        let mut lowest_score = u32::MAX;

        let creep_positions: HashMap<Position, String> = HashMap::new();
        let adjacent_positions: Vec<Position> = Vec::new();

        for pos in adjacent_positions {
            let creep_at_pos_name = creep_positions.get(&pos);
            if let Some(creep_at_pos_name) = creep_at_pos_name {
                let creep_at_pos = game_state.creeps.get(creep_at_pos_name);
                if let Some(creep_at_pos) = creep_at_pos {
                    // if fatigued, has no move parts, has already moved
                }
            }

            if avoid_positions.contains(&pos) { continue }
            if utils::general::is_exit(pos) { continue }

            let mut score: u32 = 0;
            if let Some(target_pos) = target_pos {
                score += utils::general::pos_range_euc(target_pos, &pos);
            }

            if score >= lowest_score { continue }

            lowest_score = score;
            move_pos = Some(pos);
        }

        move_pos
    }

    pub fn run_move_request(creep_name: &String) {

    }
}