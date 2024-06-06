use screeps::{BodyPart, Part, Room, RoomName, StructureSpawn, StructureType};

use crate::{
    constants::{
        creep::{BodypartCounts, CreepPart, CreepRole},
        spawning::{SpawnRequest, SpawnRequestArgs},
    },
    memory::{game_memory::GameMemory, room_memory::RoomMemory},
    room::room_ops::RoomOps,
    state::{commune::CommuneState, game::GameState, room::RoomState},
    utils::general::GeneralUtils,
};

use super::{
    spawn_request_arg_ops::SpawnRequestArgOps, spawn_request_arg_services::SpawnRequestArgServices,
};

pub struct SpawnServices;

impl SpawnServices {
    pub fn try_spawn_creeps(
        room_name: &RoomName,
        game_state: &mut GameState,
        memory: &mut GameMemory,
    ) {
        let structures = RoomOps::structures(room_name, game_state);

        let mut active_spawns: Vec<&StructureSpawn> = Vec::new();
        let mut inactive_spawns: Vec<&StructureSpawn> = Vec::new();

        for spawn in &structures.spawn {
            match spawn.spawning() {
                Some(spawning) => active_spawns.push(spawn),
                _ => inactive_spawns.push(spawn),
            }
        }

        Self::try_use_inactive_spawns(room_name, game_state, memory);
    }

    fn try_use_inactive_spawns(
        room_name: &RoomName,
        game_state: &mut GameState,
        memory: &mut GameMemory,
    ) {
        let mut inactive_spawns: Vec<&StructureSpawn> = Vec::new();

        let spawn_requests_args =
            SpawnRequestArgServices::create_spawn_request_args(room_name, game_state, memory);

        for spawn_request_args in spawn_requests_args {
            let spawn_requests = match spawn_request_args {
                SpawnRequestArgs::IndividualUniform(args) => {
                    SpawnRequestArgOps::spawn_request_individual_uniform(
                        args, room_name, game_state, memory,
                    )
                }
                SpawnRequestArgs::GroupUniform(args) => {
                    SpawnRequestArgOps::spawn_request_group_uniform(
                        &args, room_name, game_state, memory,
                    )
                }
                SpawnRequestArgs::GroupDiverse(args) => {
                    SpawnRequestArgOps::spawn_request_group_diverse(
                        &args, room_name, game_state, memory,
                    )
                }
            };

            for spawn_request in spawn_requests {
                Self::process_spawn_request(&spawn_request)
            }
        }

        for spawn in inactive_spawns {}
    }

    fn process_spawn_request(spawn_request: &SpawnRequest) {
        let body = Self::construct_body_for_spawn_request(spawn_request);
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

        body
    }

    fn find_spawn_index_for_spawn_request(
        spawn_request: &SpawnRequest,
        inactive_spawns: Vec<StructureSpawn>,
    ) -> u32 {
        let Some(spawn_target) = spawn_request.spawn_target else {
            return 0;
        };

        let (score, index) =
            GeneralUtils::find_index_with_lowest_score(&inactive_spawns, &|spawn| {
                GeneralUtils::pos_range(&screeps::HasPosition::pos(&spawn), &spawn_target)
            });

        index
    }
}
