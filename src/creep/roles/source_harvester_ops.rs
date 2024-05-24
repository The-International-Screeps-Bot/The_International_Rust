use screeps::{ObjectId, SharedCreepProperties};

use crate::{creep::owned_creep_ops::OwnedCreepOps, memory::game_memory::GameMemory, state::game::GameState};

pub struct SourceHarvesterOps;

impl SourceHarvesterOps {
    pub fn harvest_steps(creep_name: &String, game_state: &mut GameState, memory: &mut GameMemory) {

        let creep = game_state.creeps.get(creep_name).unwrap();

        let room = game_state.rooms.get(&creep.inner().room().unwrap().name()).unwrap();

        let room_memory = memory.rooms.get(&room.name()).unwrap();
        let source_id = room_memory.commune_sources[0];

        OwnedCreepOps::drop_harvest(creep_name, &source_id, game_state, memory);
    }
}