use screeps::ResourceType;
use serde::{Deserialize, Serialize};

use crate::constants::{self, general::DEFAULT_DATA_DECAY, player};

#[derive(Serialize, Deserialize)]
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

    pub fn decay_reputation(&mut self) {
        self.reputation *= DEFAULT_DATA_DECAY;
    }

    pub fn reputation_for_resources(&mut self, amount: u32, _resource_type: ResourceType) {
        self.reputation += (constants::player::reputation::RESOURCE_GIVEN * amount) as f32;
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnemyMemory {
    pub offensive_strength: f32,
    pub defensive_strength: f32,
    /// How much we want them dead
    pub hate: f32,
    /// How many rooms we think they can claim. More GCL than rooms or relatively high GCL can be considered.
    pub estimated_gcl: u32,
    /// How aggressive we think their tower code is. That is to say, how much net damage can we get away with.
    pub tower_aggressiveness: f32,
}

impl EnemyMemory {
    pub fn new() -> Self {
        Self {
            offensive_strength: 0.,
            defensive_strength: 0.,
            hate: 0.,
            estimated_gcl: 0,
            tower_aggressiveness: 0.,
        }
    }

    pub fn decay_offensive_strength(&mut self) {
        self.offensive_strength *= DEFAULT_DATA_DECAY;
    }

    pub fn decay_defensive_strength(&mut self) {
        self.defensive_strength *= DEFAULT_DATA_DECAY;
    }

    pub fn decay_hate(&mut self) {
        self.hate *= DEFAULT_DATA_DECAY;
    }
}
