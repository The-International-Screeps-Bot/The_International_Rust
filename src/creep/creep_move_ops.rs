use std::collections::{HashMap, HashSet};

use screeps::{Creep, HasPosition, Position};

use crate::{constants::general::{GeneralError, GeneralResult}, state::game::GameState, utils::general::GeneralUtils};

pub struct CreepMoveOps;

impl CreepMoveOps {
    pub fn create_move_request(creep: &Creep, origin: &Position) {

    }

    fn assign_move_request(creep: &Creep) {

    }

    pub fn try_run_move_request(creep: &Creep, game_state: &GameState, avoid_positions: &mut HashSet<Position>) -> GeneralResult {

        let creep_positions: HashMap<Position, String> = HashMap::new();

        let target_pos = creep.pos();

        let move_pos = Self::find_move_coord(creep, game_state, &avoid_positions, Some(&target_pos));
        let Some(move_pos) = move_pos else {
            return GeneralResult::Fail
        };

        let creep_at_pos_name = creep_positions.get(&move_pos);
        if let Some(creep_at_pos_name) = creep_at_pos_name {
            let creep_at_pos = game_state.creeps.get(creep_at_pos_name);
            if let Some(creep_at_pos) = creep_at_pos {
                avoid_positions.insert(creep.pos());
                avoid_positions.insert(move_pos);

                let move_result = Self::try_run_move_request(creep, game_state, avoid_positions);
                if move_result != GeneralResult::Success {
                    return move_result
                }
            }
        }

        Self::run_move_request(creep);
        GeneralResult::Success
    }

    fn find_move_coord(creep: &Creep, game_state: &GameState, avoid_positions: &HashSet<Position>, target_pos: Option<&Position>) -> Option<Position> {

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
            if GeneralUtils::is_exit(pos) { continue }

            let mut score: u32 = 0;
            if let Some(target_pos) = target_pos {
                score += GeneralUtils::pos_range_euc(target_pos, &pos);
            }

            if score >= lowest_score { continue }

            lowest_score = score;
            move_pos = Some(pos);
        }

        move_pos
    }

    pub fn run_move_request(creep: &Creep) {

    }
}