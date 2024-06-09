use screeps::{Room, RoomName};

use crate::{
    constants::{
        creep::{CreepPart, CreepRole},
        spawning::{spawn_priority_bounds, GroupDiverseSpawnRequestArgs, GroupUniformSpawnRequestArgs, SpawnRequestArgs},
    },
    memory::{creep_memory::CreepMemory, game_memory::GameMemory, room_memory::RoomMemory},
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
    let commune_state = game_state.commune_states.get(room_name).unwrap();

    if commune_state.spawn_energy_capacity > 550 {
        spawn_request_args.push(SpawnRequestArgs::GroupUniform(
            GroupUniformSpawnRequestArgs {
                role: CreepRole::SourceHarvester,
                default_parts: vec![CreepPart::Move],
                extra_parts: vec![CreepPart::Work],
                parts_quota: 20,
                min_cost_per_creep: 100,
                max_cost_per_creep: None,
                memory_additions: CreepMemory::new(room_name),
                priority: spawn_priority_bounds::SOURCE_HARVESTER.0,
                max_creeps: None,
                threshold: None,
                spawn_target: None,
            },
        ));

        return;
    };

    spawn_request_args.push(SpawnRequestArgs::GroupUniform(
        GroupUniformSpawnRequestArgs {
            role: CreepRole::SourceHarvester,
            default_parts: vec![CreepPart::Carry],
            extra_parts: vec![CreepPart::Move, CreepPart::Work, CreepPart::Work],
            parts_quota: 4,
            min_cost_per_creep: 100,
            max_cost_per_creep: None,
            memory_additions: CreepMemory::new(room_name),
            priority: spawn_priority_bounds::SOURCE_HARVESTER.0,
            max_creeps: None,
            threshold: None,
            spawn_target: None,
        },
    ));
}

fn hauler_args(room: &Room, request_args: &mut Vec<SpawnRequestArgs>, game_state: &GameState) {
    let _ = request_args;
}
