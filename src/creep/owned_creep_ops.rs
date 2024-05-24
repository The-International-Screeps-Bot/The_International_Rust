use log::{info, warn};
use screeps::{ErrorCode, HasPosition, ObjectId, SharedCreepProperties, Source};

use crate::{
    constants::creep::{CreepOperationResult, CreepRole},
    creep::owned_creep::OwnedCreep,
    memory::game_memory::GameMemory,
    state::game::GameState,
};

use super::{creep_move_ops::CreepMoveOps, roles::source_harvester_ops::SourceHarvesterOps};

pub struct OwnedCreepOps;

/// Only creeps the bot owns should use these functions
impl OwnedCreepOps {
    // The running of creep roles should be more dynamic and seperated. For example, a function to run harvesting for all harvesters, etc.
    pub fn run_role(creep_name: &String, game_state: &mut GameState, memory: &mut GameMemory) {
        let creep = game_state.creeps.get(creep_name).unwrap();

        let Some(creep_memory) = memory.creeps.get(&creep.inner().name()) else {
            return;
        };

        match creep_memory.role {
            CreepRole::SourceHarvester => {
                SourceHarvesterOps::harvest_steps(creep_name, game_state, memory);
            }
            _ => {
                info!("no role provided for {}", creep.inner().name());
            }
        }
    }

    pub fn drop_harvest(
        creep_name: &String,
        source_id: &ObjectId<Source>,
        game_state: &mut GameState,
        memory: &mut GameMemory,
    ) -> CreepOperationResult {
        let creep = game_state.creeps.get(creep_name).unwrap();

        let Some(source) = source_id.resolve() else {
            warn!("source id {} not found", source_id);
            return CreepOperationResult::Exception;
        };

        let source_pos = source.pos();
        if creep.inner().pos().is_near_to(source_pos) {
            return match creep.inner().harvest(&source) {
                Ok(()) | Err(ErrorCode::NotEnough) => CreepOperationResult::Fail,
                Err(e) => {
                    warn!(
                        "creep {} unexpected error {:?} when harvesting",
                        creep.inner().name(),
                        e
                    );
                    CreepOperationResult::Exception
                }
            };
        } else {
            // The creep needs to move to the source to harvest it.
            CreepMoveOps::create_move_request(creep_name, &source_pos, game_state, memory);
            CreepOperationResult::InProgress
        }
    }

    pub fn clean_creep_memories(game_state: &GameState, memory: &mut GameMemory) {
        info!("running memory cleanup");

        memory
            .creeps
            .retain(|creep_name, _creep| game_state.creeps.contains_key(creep_name));
    }
}
