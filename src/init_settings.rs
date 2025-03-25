use crate::{settings::Settings, state::game::GameState};

pub fn init_settings(settings: &mut Settings, game_state: &GameState) {

    // Omnipresent settings
    
    // settings.log_filter = log::LevelFilter::Trace;

    // MMO or Season
    if game_state.shard.contains("shard") {
        settings.allies.insert(String::from("PandaMaster"));
        return
    }

    if game_state.shard == "botarena" {
        return
    }
}