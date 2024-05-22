use screeps::{Room, MAX_CREEP_SIZE};

use crate::{constants::spawning::{SpawnRequestArgs, SpawnRequests}, state::{commune::CommuneState, room::RoomState}};

pub struct SpawnRequestArgOps;

impl SpawnRequestArgOps {
    pub fn create_spawn_requests_individual_uniform(room: &Room, room_state: &mut RoomState, commune_state: &mut CommuneState, args: &SpawnRequestArgs) {

        let spawn_requests: SpawnRequests = Vec::new();


    }

    pub fn create_spawn_requests_group_diverse(room: &Room, room_state: &mut RoomState, commune_state: &mut CommuneState, args: &SpawnRequestArgs) {

        let spawn_requests: SpawnRequests = Vec::new();

        let max_cost_per_creep = Self::get_max_cost_per_creep(commune_state, args);

        // let total_extra_parts = args.extra_parts.len() * args.pa
        // let max_parts_per_creep = (MAX_CREEP_SIZE, ).max()

    }

    // Not fully sold on needing group uniform

    pub fn create_spawn_requests_group_uniform(room: &Room, room_state: &mut RoomState, commune_state: &mut CommuneState, args: &SpawnRequestArgs) {

        if args.extra_parts.len() == 0 {
            panic!("No extra parts provided");
            return
        }

        let spawn_requests: SpawnRequests = Vec::new();

        let max_cost_per_creep = Self::get_max_cost_per_creep(commune_state, args);

        
    }

    fn get_max_cost_per_creep(commune_state: &mut CommuneState, args: &SpawnRequestArgs) -> u32 {
        match args.max_cost_per_creep {
            Some(cost) => {
                if cost < args.min_cost_per_creep {
                    panic!("Max cost per creep cannot be less than min cost per creep");
                    return 0
                }

                if cost > commune_state.spawn_energy_capacity {
                    panic!("Max cost per creep cannot be greater than spawn energy capacity");
                    return 0
                }

                0
            }
            None => {
                commune_state.spawn_energy_capacity
            }
        }
    }

    fn create_spawn_request(args: &SpawnRequestArgs) {

    }
}