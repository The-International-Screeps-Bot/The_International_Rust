use screeps::ResourceType;
use serde::{Deserialize, Serialize};

use crate::{constants::{self, general::DEFAULT_DATA_DECAY, player}, state::game::GameState};

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

    pub fn decay_offensive_strength(&mut self, game_state: &mut GameState) {
        self.offensive_strength *= DEFAULT_DATA_DECAY / game_state.intervals.decay_player_data as f32;
    }

    pub fn decay_defensive_strength(&mut self, game_state: &mut GameState) {
        self.defensive_strength *= DEFAULT_DATA_DECAY / game_state.intervals.decay_player_data as f32;
    }

    pub fn decay_hate(&mut self, game_state: &mut GameState) {
        self.hate *= DEFAULT_DATA_DECAY / game_state.intervals.decay_player_data as f32;
    }
}
