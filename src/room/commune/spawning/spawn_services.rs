use log::{debug, info};
use screeps::{BodyPart, Part, Room, RoomName, SpawnOptions, StructureSpawn, StructureType};
use wasm_bindgen::JsValue;
use web_sys::console::info;

use crate::{
    constants::{
        creep::{
            BodypartCounts, CreepPart, CreepRole, PriorityCreepPart, PARTS_BY_PRIORITY,
            PARTS_BY_PRIORITY_PART,
        },
        general::{FlowResult, GeneralError, GeneralResult},
        spawning::{SpawnRequest, SpawnRequestArgs},
    },
    international::collective_ops::new_creep_id,
    memory::{game_memory::GameMemory, room_memory::RoomMemory},
    room::room_ops,
    state::{commune::CommuneState, game::GameState, room::RoomState},
    utils::{self, general::GeneralUtils},
};

use super::{
    spawn_request_arg_ops, spawn_request_arg_services,
};

pub fn try_spawn_creeps(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {
    let spawns = room_ops::structures_by_type(room_name, game_state).spawn.clone();

    let mut active_spawns: Vec<&StructureSpawn> = Vec::new();
    let mut inactive_spawns: Vec<&StructureSpawn> = Vec::new();

    for spawn in &spawns {
        match spawn.spawning() {
            Some(spawning) => active_spawns.push(spawn),
            _ => inactive_spawns.push(spawn),
        }
    }

    info!("A");

    try_use_inactive_spawns(room_name, game_state, memory, &mut inactive_spawns);
}

fn try_use_inactive_spawns(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
    inactive_spawns: &mut Vec<&StructureSpawn>,
) {
    
    if inactive_spawns.is_empty() {
        return;
    }

    let spawn_requests_args =
        spawn_request_arg_services::create_spawn_request_args(room_name, game_state, memory);

    let room = game_state.rooms.get(room_name).unwrap();

    let mut spawn_energy_remaining = room.energy_available();

    info!("B");
    debug!("spawn_requests_args: {:?}", spawn_requests_args);

    for spawn_request_args in spawn_requests_args {
        let mut spawn_requests = match spawn_request_args {
            SpawnRequestArgs::IndividualUniform(args) => {
                spawn_request_arg_ops::spawn_request_individual_uniform(
                    args, room_name, game_state, memory,
                )
            }
            SpawnRequestArgs::GroupUniform(args) => {
                spawn_request_arg_ops::spawn_request_group_uniform(
                    &args, room_name, game_state, memory,
                )
            }
            SpawnRequestArgs::GroupDiverse(args) => {
                spawn_request_arg_ops::spawn_request_group_diverse(
                    &args, room_name, game_state, memory,
                )
            }
        };

        println!("Spawn requests: {:?}", spawn_requests);

        spawn_requests.sort_by(|a, b| {
            a.priority.partial_cmp(&b.priority).unwrap()
        });

        for spawn_request in spawn_requests {

            let spawn_request_result = process_spawn_request(spawn_request, inactive_spawns, game_state, memory, spawn_energy_remaining);
            match spawn_request_result {
                Ok((spawn_index, cost)) => {
                    spawn_energy_remaining -= cost;

                    inactive_spawns.remove(spawn_index);

                    if inactive_spawns.is_empty() {
                        break;
                    }
                }
                Err(FlowResult::Stop) => break,
                Err(FlowResult::Continue) => continue,
            }


        }
    }
}

fn process_spawn_request(
    spawn_request: SpawnRequest,
    inactive_spawns: &mut [&StructureSpawn],
    game_state: &mut GameState,
    memory: &mut GameMemory,
    spawn_enery_available: u32,
) -> Result<(usize, u32), FlowResult> {

    let cost = spawn_request.cost;
    if spawn_enery_available < cost {
        return Err(FlowResult::Stop);
    }

    let body = construct_body_for_spawn_request(&spawn_request);

    // TODO: the index should be selected based on the most optimal spawn to spawn from
    let spawn = inactive_spawns[0];
    let spawn_index = 0;

    let custom_id = new_creep_id(game_state, memory).ok().unwrap();
    // let name = format!("{:?}_{}", spawn_request.role, custom_id);
    let name = format!("{}", custom_id);

    // let energy_structures;

    let spawn_result = spawn.spawn_creep_with_options(
        body.as_slice(),
        name.as_str(),
        // Need energy structures
        &SpawnOptions::new().dry_run(false),
    );

    let Ok(spawn_result) = spawn_result else {
        log::error!("Failed to spawn creep: {:?}", spawn_result);
        return Err(FlowResult::Stop);
    };

    // If the spawning was successful

    // Insert the creep's memory directly
    memory.creeps.insert(name, spawn_request.memory);

    Ok((spawn_index, cost))
}

fn construct_body_for_spawn_request(spawn_request: &SpawnRequest) -> Vec<Part> {
    let mut body: Vec<Part> = Vec::new();

    // If the creep is only made up of carry and move
    if spawn_request.cost
        == spawn_request.body_part_counts[CreepPart::Carry] * Part::cost(Part::Carry)
            + spawn_request.body_part_counts[CreepPart::Move] * Part::cost(Part::Move)
    {
        let ratio = spawn_request.body_part_counts[CreepPart::Carry]
            / spawn_request.body_part_counts[CreepPart::Move];

        let mut i: i32 = -1;
        while i < spawn_request.body_part_counts[CreepPart::Carry] as i32 - 1 {
            body.push(Part::Carry);
            if i % ratio as i32 == 0 {
                body.push(Part::Move);
            }

            i += 1;
        }

        return body;
    }

    let mut end_parts: Vec<Part> = Vec::new();

    PARTS_BY_PRIORITY_PART.with(|parts_by_priority_part| {
        for (index, priority_part) in PARTS_BY_PRIORITY.iter().enumerate() {
            let part = parts_by_priority_part[*priority_part];

            let bodypart_count = spawn_request.body_part_counts[part];
            if bodypart_count == 0 {
                continue;
            }

            let mut skip_end_part: bool = false;
            let mut priority_parts_count = 0;

            match priority_part {
                PriorityCreepPart::RangedAttack => {
                    priority_parts_count = bodypart_count;
                    skip_end_part = true;
                }
                PriorityCreepPart::Attack => {
                    priority_parts_count = (bodypart_count).div_ceil(2);
                    skip_end_part = true;
                }
                PriorityCreepPart::Tough => {
                    priority_parts_count = (bodypart_count).div_ceil(2);
                    skip_end_part = true;
                }
                PriorityCreepPart::SecondaryAttack => {
                    priority_parts_count = (bodypart_count).div_floor(2);
                    skip_end_part = true;
                }
                PriorityCreepPart::SecondaryTough => {
                    priority_parts_count = (bodypart_count).div_floor(2);
                    skip_end_part = true;
                }
                _ => {
                    priority_parts_count = bodypart_count - 1;
                }
            }

            for _ in 0..priority_parts_count {
                body.push(part.to_part());
            }

            if skip_end_part {
                continue;
            }

            end_parts.push(part.to_part());
        }
    });

    // put end parts at the end of body
    body.append(&mut end_parts);

    body
}

fn find_spawn_index_for_spawn_request(
    spawn_request: &SpawnRequest,
    inactive_spawns: Vec<StructureSpawn>,
) -> u32 {
    let Some(spawn_target) = spawn_request.spawn_target else {
        return 0;
    };

    let (score, index) = utils::general::find_index_with_lowest_score(&inactive_spawns, &|spawn| {
        utils::general::pos_range(&screeps::HasPosition::pos(&spawn), &spawn_target)
    });

    index
}
