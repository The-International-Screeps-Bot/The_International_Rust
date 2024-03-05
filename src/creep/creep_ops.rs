use log::{info, warn};
use screeps::{Creep, ErrorCode, HasPosition, ObjectId, SharedCreepProperties, Source};

use crate::{constants::creep::{CreepOperationResult, CreepRole}, memory::game_memory::GameMemory, state::game::GameState};

use super::creep_move_ops::CreepMoveOps;

pub struct CreepOps;

/// Only creeps the bot owns should use these functions
impl CreepOps {
    pub fn run_role(creep: &Creep, game_state: &GameState, memory: &mut GameMemory) {

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

    pub fn drop_harvest(creep: &Creep, source_id: &ObjectId<Source>) -> CreepOperationResult {
        let creep_pos = creep.pos();
        let Some(source) = source_id.resolve() else {
            warn!("source id {} not found", source_id);
            return CreepOperationResult::Exception;
        };

        let source_pos = source.pos();

        if creep_pos.is_near_to(source.pos()) {
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
        }

        // The creep is not in harvest range for the source

        let _ = CreepMoveOps::create_move_request(creep, &source_pos);
        CreepOperationResult::InProgress
    }

    pub fn clean_creep_memories(game_state: &GameState, memory: &mut GameMemory) {

        info!("running memory cleanup");
    
        let _ = &memory.creeps.retain(|creep_name, _creep| {

            game_state.creeps.contains_key(creep_name)
        });
    }
}
