use screeps::Room;

use crate::{constants::spawning::{SpawnRequestArgs, SpawnRequests}, state::game::GameState};

pub struct SpawnRequestArgServices;


impl SpawnRequestArgServices { 
    // Construct args... not spawn requests
    pub fn create_spawn_request_args(room: &Room, game_state: &mut GameState) {

        let request_args: Vec<SpawnRequestArgs> = Vec::new();


    }

    fn harvester_args(room: &Room, request_args: &mut Vec<SpawnRequestArgs>, game_state: &mut GameState) {

        
    }

    fn hauler_args(room: &Room, request_args: &mut Vec<SpawnRequestArgs>, game_state: &mut GameState) {

    }
}