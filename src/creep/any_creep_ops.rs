use screeps::{BodyPart, Creep};

use crate::{
    constants::creep::{CreepPart, CreepParts, CreepPartsByType, CREEP_PARTS_BY_TYPE},
    state::{creep::CreepState, game::GameState},
};

// pub fn get_parts_by_type(
//     body: &Creep,
//     creep_name: &str,
//     game_state: &mut GameState,
// ) -> CreepPartsByType {
//     // let binding = game_state.creep_states.get_mut(&creep_name.to_string());
//     // let binding = CreepState::new(creep_name);
//     // let creep_state = binding.get_or_insert(&mut binding);

//     // let creep_state = game_state.creep_states.get(creep_name).unwrap_or_default(&CreepState::new(creep_name));

//     let creep_state = game_state
//         .creep_states
//         .entry(creep_name.to_string())
//         .or_insert(CreepState::new(creep_name));

//     // let Some(mut creep_state) = game_state.creep_states.get(&creep_name.to_string()) else {
//     //     creep_state
//     // }

//     if let Some(parts) = creep_state.parts_by_type {
//         return parts;
//     };

//     let parts = CREEP_PARTS_BY_TYPE.with(|parts| {
//         let mut parts = parts.clone();

//         for body_part in body {
//             parts[CreepPart::from_part(&body_part.part())] += 1;
//         }

//         parts
//     });

//     parts
// }
