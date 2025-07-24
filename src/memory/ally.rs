use screeps::ResourceType;
use serde::{Deserialize, Serialize};

use crate::{constants::{self, general::DEFAULT_DATA_DECAY, player}, state::game::GameState};

#[derive(Serialize, Deserialize, Debug)]
pub struct AllyMemory {
    /// reputation is used to measure the synergy of players with us. If players provide us lots of resource, assistance, etc. we will recognize that and consider it in our own decision making
    pub reputation: f32,
    /// Wether or not the player seems to have public ramparts. This is decided base on, if not yet decided, if there are no other player's creeps in the room and there are still no public ramparts. It's rough, but should get it mostly right
    /// Avoid travel through rooms where ramparts are not public
    pub public_ramparts: Option<bool>,
}

impl AllyMemory {
    pub fn new() -> Self {
        Self { reputation: 0., public_ramparts: None }
    }

    pub fn decay_reputation(&mut self, game_state: &mut GameState) {
        self.reputation *= DEFAULT_DATA_DECAY / game_state.intervals.decay_player_data as f32;
    }

    pub fn reputation_for_resources(&mut self, amount: u32, _resource_type: ResourceType) {
        self.reputation += (constants::player::reputation::RESOURCE_GIVEN * amount) as f32;
    }
}