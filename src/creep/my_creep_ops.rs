use log::{info, warn};
use screeps::{ErrorCode, HasPosition, ObjectId, Part, SharedCreepProperties, Source};

use crate::{
    constants::creep::{
        CreepOperationResult, CreepPart, CreepParts, CreepPartsByType, CreepRole,
        CREEP_PARTS_BY_TYPE,
    },
    creep::my_creep::MyCreep,
    memory::game_memory::GameMemory,
    state::{game::GameState, my_creep::MyCreepState},
};

use super::{
    creep_move_ops::CreepMoveOps,
    roles::source_harvester_ops::{self},
};

// Transfer these over to MyCreep, if we are commiting to the types decrepancy

// The running of creep roles should be more dynamic and seperated. For example, a function to run harvesting for all harvesters, etc.
// pub fn run_role(creep_name: &String, game_state: &mut GameState, memory: &mut GameMemory) {
//     let creep = game_state.creeps.get(creep_name).unwrap();

//     let Some(creep_memory) = memory.creeps.get(&creep.inner().name()) else {
//         return;
//     };

//     match creep_memory.role {
//         CreepRole::SourceHarvester => {
//             source_harvester_ops::harvest_steps(creep_name, game_state, memory);
//         }
//         _ => {
//             info!("no role provided for {}", creep.inner().name());
//         }
//     }
// }

pub fn drop_harvest(
    creep_name: &String,
    source: &Source,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> CreepOperationResult {
    let creep = game_state.creeps.get(creep_name).unwrap();

    let source_pos = source.pos();
    if creep.inner().pos().is_near_to(source_pos) {
        return match creep.inner().harvest(source) {
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

pub fn get_parts(creep_name: &str, game_state: &mut GameState) -> Vec<Part> {
    {
        let creep_state = game_state.my_creep_states.get(creep_name).unwrap();

        if let Some(parts) = &creep_state.parts {
            return parts.to_vec();
        };
    }

    let creep = game_state.creeps.get(creep_name).unwrap();
    let parts: Vec<Part> = creep
        .inner()
        .body()
        .iter()
        .map(|body_part| body_part.part())
        .collect();

    let creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();
    creep_state.parts = Some(parts.to_vec());

    parts
}

pub fn get_parts_by_type(creep_name: &str, game_state: &mut GameState) -> CreepPartsByType {
    // let binding = game_state.creep_states.get_mut(&creep_name.to_string());
    // let binding = CreepState::new(creep_name);
    // let creep_state = binding.get_or_insert(&mut binding);

    // let creep_state = game_state.creep_states.get(creep_name).unwrap_or_default(&CreepState::new(creep_name));
    {
        let creep_state = game_state.my_creep_states.get(creep_name).unwrap();

        if let Some(parts_by_type) = creep_state.parts_by_type {
            return parts_by_type;
        };
    }
    // let Some(mut creep_state) = game_state.creep_states.get(&creep_name.to_string()) else {
    //     creep_state
    // }

    let creep = game_state.creeps.get(creep_name).unwrap();
    let parts = get_parts(creep_name, game_state);

    let parts_by_type = CREEP_PARTS_BY_TYPE.with(|parts_by_type| {
        let mut parts_by_type = parts_by_type.clone();

        for part in parts {
            parts_by_type[CreepPart::from_part(&part)] += 1;
        }

        parts_by_type
    });

    let creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();
    creep_state.parts_by_type = Some(parts_by_type);

    parts_by_type
}
