use crate::{constants::general::DEFAULT_DATA_DECAY, memory::{game_memory::GameMemory, player::AllyMemory}, settings::Settings};

pub fn decay_metrics(memory: &mut GameMemory) {

    let player_names: Vec<String> = memory.allies.keys().cloned().collect();

    for player_name in player_names {
        let ally_memory = memory.allies.get_mut(&player_name).unwrap();

        ally_memory.decay_reputation();
    }
}

pub fn init_allies(settings: &Settings, memory: &mut GameMemory) {
    for player_name in &settings.allies {

        memory.allies.insert(player_name.clone(), AllyMemory::new());
    }
}