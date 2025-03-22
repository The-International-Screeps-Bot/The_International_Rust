use screeps::{Room, RoomName, ENERGY_REGEN_TIME, HARVEST_POWER, SOURCE_ENERGY_CAPACITY};

use crate::{
    constants::{
        creep::{CreepPart, CreepRole},
        spawning::{
            spawn_priority_bounds, GroupDiverseSpawnRequestArgs, GroupUniformSpawnRequestArgs,
            SpawnRequestArgs,
        },
    },
    memory::{
        creep_memory::CreepMemory,
        game_memory::GameMemory,
        room_memory::{self, RoomMemory},
    },
    state::{commune::CommuneState, game::GameState, room::RoomState},
};

// Construct args... not spawn requests
pub fn create_spawn_request_args(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> Vec<SpawnRequestArgs> {
    let mut spawn_request_args: Vec<SpawnRequestArgs> = Vec::new();

    harvester_args(&mut spawn_request_args, room_name, game_state, memory);

    spawn_request_args
}

fn harvester_args(
    spawn_request_args: &mut Vec<SpawnRequestArgs>,
    room_name: &RoomName,
    game_state: &GameState,
    memory: &mut GameMemory,
) {
    let Some(harvestable_room_memory) = memory.harvestable_rooms.get(room_name) else {
        return;
    };
    let base_priority = spawn_priority_bounds::SOURCE_HARVESTER.0;
    let commune_state = game_state.commune_states.get(room_name).unwrap();

    for (source_index, source_position) in harvestable_room_memory.source_positions.iter().enumerate() {

        // source harvest need derived from how many work parts
        let work_need = SOURCE_ENERGY_CAPACITY / ENERGY_REGEN_TIME / HARVEST_POWER + 1;
        let Some(work_have) = commune_state.source_harvest_strengths.get(source_index) else {
            continue;
        };

        let work_quota = work_need.saturating_sub(*work_have);
        if work_quota == 0 {
            continue;
        }

        let priority = base_priority + source_index as f32;

        let role = CreepRole::SourceHarvester;

        if commune_state.spawn_energy_capacity > 550 {
            let default_parts = vec![CreepPart::Move];
            let extra_parts = vec![CreepPart::Work];

            spawn_request_args.push(SpawnRequestArgs::GroupUniform(
                GroupUniformSpawnRequestArgs {
                    role: CreepRole::SourceHarvester,
                    default_parts,
                    extra_parts,
                    extra_parts_quota: work_quota,
                    min_cost_per_creep: 100,
                    max_cost_per_creep: None,
                    memory_additions: {
                        let mut creep_memory = CreepMemory::new(role, *room_name);
                        creep_memory.source_index = Some(source_index);
                        creep_memory
                    },
                    priority,
                    max_creeps: None,
                    threshold: None,
                    spawn_target: Some(*source_position),
                },
            ));

            continue;
        };

        spawn_request_args.push(SpawnRequestArgs::GroupUniform(
            GroupUniformSpawnRequestArgs {
                role: CreepRole::SourceHarvester,
                default_parts: vec![CreepPart::Carry],
                extra_parts: vec![CreepPart::Move, CreepPart::Work, CreepPart::Work],
                extra_parts_quota: work_quota / 2,
                min_cost_per_creep: 100,
                max_cost_per_creep: None,
                memory_additions: {
                    let mut creep_memory = CreepMemory::new(role, *room_name);
                    creep_memory.source_index = Some(source_index);
                    creep_memory
                },
                priority,
                max_creeps: None,
                threshold: None,
                spawn_target: Some(*source_position),
            },
        ));
    }
}

fn hauler_args(room: &Room, request_args: &mut Vec<SpawnRequestArgs>, game_state: &GameState) {
    let _ = request_args;
}
