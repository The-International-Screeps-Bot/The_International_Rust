use log::{info, warn};
use screeps::{ErrorCode, HasPosition, ObjectId, SharedCreepProperties, Source};

use crate::{
    constants::creep::CreepOperationResult, creep::owned_creep::OwnedCreep,
    memory::game_memory::GameMemory, state::game::GameState,
};

use super::creep_move_ops::CreepMoveOps;

pub struct OwnedCreepOps;

/// Only creeps the bot owns should use these functions
impl OwnedCreepOps {
    pub fn run_role(creep: &OwnedCreep, game_state: &GameState, memory: &mut GameMemory) {
        let creep = creep.inner();
        let Some(creep_memory) = memory.creeps.get(&creep.name()) else {
            return;
        };

        match creep_memory.role {
            // CreepRole::Harvester => {
            //     HarvesterOps::run(creep);
            // }
            _ => {
                info!("no role provided for {}", creep.name());
            }
        }
    }

    pub fn drop_harvest(creep: &OwnedCreep, source_id: &ObjectId<Source>) -> CreepOperationResult {
        let creep = creep.inner();
        let creep_pos = creep.pos();
        let Some(source) = source_id.resolve() else {
            warn!("source id {} not found", source_id);
            return CreepOperationResult::Exception;
        };

        let source_pos = source.pos();
        if creep_pos.is_near_to(source_pos) {
            return match creep.harvest(&source) {
                Ok(()) | Err(ErrorCode::NotEnough) => CreepOperationResult::Fail,
                Err(e) => {
                    warn!(
                        "creep {} unexpected error {:?} when harvesting",
                        creep.name(),
                        e
                    );
                    CreepOperationResult::Exception
                }
            };
        } else {
            // The creep needs to move to the source to harvest it.
            CreepMoveOps::create_move_request(creep, &source_pos);
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
