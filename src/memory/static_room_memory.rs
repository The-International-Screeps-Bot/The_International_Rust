use std::collections::HashSet;

use screeps::{
    HasPosition, Mineral, OwnedStructureProperties, Position, RoomName, constants, find,
};
use serde::{Deserialize, Serialize};

use crate::{constants::general::GeneralError, room::room_ops, state::game::GameState};

use super::{game_memory::GameMemory, room_memory::InvaderCodeInfo};

#[derive(Serialize, Deserialize, Debug)]
/// Used for all rooms that have a controller
pub struct ClaimableRoomMemory {
    /// The name of the owner if it isn't owned by us
    pub non_me_owner: Option<String>,
    /// Wether or not we have claimed this room
    pub my_claim: Option<bool>,
    // #[serde(serialize_with="serialize_pos")]
    pub controller_pos: Position,
}

impl ClaimableRoomMemory {
    pub fn new(
        room_name: &RoomName,
        game_state: &mut GameState,
        memory: &GameMemory,
    ) -> Result<Self, GeneralError> {
        let Some(room) = game_state.rooms.get(room_name) else {
            return Err(GeneralError::Fail);
        };

        // Controller pos

        let Some(controller) = room.controller() else {
            return Err(GeneralError::Fail);
        };
        let controller_pos = controller.pos();

        // Controller owner

        let mut my_claim = None;
        let mut non_me_owner = None;

        if let Some(owner) = controller.owner() {
            let username = owner.username();
            if username == memory.me {
                my_claim = Some(true);
            } else {
                non_me_owner = Some(username);
            }
        }

        Ok(Self {
            controller_pos,
            my_claim: None,
            non_me_owner: None,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeeperRoomMemory {
    pub keeper_positions: HashSet<Position>,
    pub invader_core_info: Option<InvaderCodeInfo>,
    pub keeper_lair_positions: Vec<Position>,
}

impl KeeperRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Self {
        let room = game_state.rooms.get(room_name).unwrap();

        // Keeper lair positions

        let keeper_lairs = &room_ops::structures_by_type(room_name, game_state).keeper_lair;
        let keeper_lair_positions = keeper_lairs
            .iter()
            .map(|keeper_lair| keeper_lair.pos())
            .collect();

        Self {
            keeper_positions: HashSet::new(),
            keeper_lair_positions,
            invader_core_info: None,
        }
    }
}