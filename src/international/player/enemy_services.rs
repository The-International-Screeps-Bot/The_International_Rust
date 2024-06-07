use crate::{constants::general::DEFAULT_DATA_DECAY, memory::{game_memory::GameMemory, player::EnemyMemory}};

pub fn decay_metrics(memory: &mut GameMemory) {

    let player_names: Vec<String> = memory.enemies.keys().cloned().collect();

    for player_name in player_names {
        let enemy_memory = memory.enemies.get_mut(&player_name).unwrap();

        enemy_memory.decay_offensive_strength();
        enemy_memory.decay_defensive_strength();
        enemy_memory.decay_hate();
    }
}