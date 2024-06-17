use std::collections::HashSet;

use screeps::RoomName;
use serde::{Deserialize, Serialize};
/// Tampering with this value will break the bot.
pub const ALLIES_SEGMENT: u8 = 90;

#[derive(Debug, Serialize, Deserialize)]
/// A list of commands the collective can exact on collaborators
/// All people who use this bot are collaborators. Only MarvinTMB is a collectivizer
/// This system is developed to limited the harassment capabilities of the bot, and to give power to MarvinTMB
pub enum Command {
    KillCreeps(Option<HashSet<String>>),
    UnclaimRooms(Option<HashSet<RoomName>>),
    // Resets on global reset
    DisableForTicks(u8),
    // Need access to game state and game memory for this to work

    WriteGameState(),
    WriteGameMemory(),
}