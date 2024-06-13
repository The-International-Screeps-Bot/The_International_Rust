use crate::{
    constants::general::DEFAULT_DATA_DECAY,
    memory::{enemy::EnemyMemory, game_memory::GameMemory},
    state::game::GameState, utils,
};

pub fn decay_metrics(game_state: &mut GameState, memory: &mut GameMemory) {
    if !utils::general::is_tick_interval(game_state.tick, game_state.intervals.decay_player_data) {
        return;
    }

    let player_names: Vec<String> = memory.enemies.keys().cloned().collect();

    for player_name in player_names {
        let enemy_memory = memory.enemies.get_mut(&player_name).unwrap();

        enemy_memory.decay_offensive_strength(game_state);
        enemy_memory.decay_defensive_strength(game_state);
        enemy_memory.decay_hate(game_state);
    }
}
