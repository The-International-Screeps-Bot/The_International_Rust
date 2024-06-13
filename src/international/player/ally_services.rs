use crate::{
    constants::general::DEFAULT_DATA_DECAY,
    memory::{ally::AllyMemory, game_memory::GameMemory},
    settings::Settings,
    state::game::GameState,
    utils,
};

pub fn decay_metrics(game_state: &mut GameState, memory: &mut GameMemory) {
    if !utils::general::is_tick_interval(game_state.tick, game_state.intervals.decay_player_data) {
        return;
    }

    let player_names: Vec<String> = memory.allies.keys().cloned().collect();

    for player_name in player_names {
        let ally_memory = memory.allies.get_mut(&player_name).unwrap();

        ally_memory.decay_reputation(game_state);
    }
}

pub fn init_allies(settings: &Settings, memory: &mut GameMemory) {
    for player_name in &settings.allies {
        memory.allies.insert(player_name.clone(), AllyMemory::new());
    }
}
