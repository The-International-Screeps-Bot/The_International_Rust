use std::{collections::HashSet, default};

use enum_map::Enum;
use screeps::{
    FindConstant, HasPosition, InterShardPortalDestination, ObjectId, PortalDestination, Position,
    Room, RoomCoordinate, RoomName, Source, constants, find,
    game::{self, map::RoomStatus},
};
use serde::{Deserialize, Serialize};

use crate::{
    constants::general::{GeneralError, GeneralResult},
    room::room_ops::{self, find_room_type},
    state::game::GameState,
};

use super::game_memory::GameMemory;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomMemory {
    pub room_type: StaticRoomType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger: Option<u32>,
    pub last_scout: u32,
}

impl RoomMemory {
    pub fn new(
        room_name: &RoomName,
        game_state: &mut GameState,
        memory: &mut GameMemory,
    ) -> Result<Self, GeneralError> {
        let room_type = find_room_type(room_name, game_state, memory)?;

        Ok(Self {
            room_type,
            last_scout: game_state.tick,
            danger: None,
        })
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug, Enum)]
/// Distinct descriptions of rooms that implies certain properties and does not change over time
pub enum StaticRoomType {
    /// Rooms we can potentially claim
    #[serde(rename = "0")]
    Claimable,
    /// The rooms surrounding the center of a sector that potentially contain portals
    #[serde(rename = "1")]
    Keeper,
    /// The center room of a sector
    #[serde(rename = "2")]
    Center,
    /// Rooms bordering sectors, excluding corners
    #[serde(rename = "3")]
    CardinalHighway,
    /// Rooms bordering sectors that are corners and potentially contain portals
    #[serde(rename = "4")]
    Intersection,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HighwayRoomMemory {
    pub deposits: Vec<u32>,
    pub power_banks: Vec<u32>,
}

impl HighwayRoomMemory {
    pub fn new() -> Self {
        Self {
            deposits: Vec::new(),
            power_banks: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommuneRoomMemory {
    /// The highest controller level the room has had without loosing ownership (implied by commune memory existing)
    highest_rcl: u8,
}

impl CommuneRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Result<Self, GeneralError> {
        
        let Some(room) = game_state.rooms.get(room_name) else {
            return Err(GeneralError::Fail);
        };

        let controller = room.controller().unwrap();
        let rcl = controller.level();
        
        Ok(Self {
            highest_rcl: rcl,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// General remote room memory
pub struct RemoteRoomMemory {
    pub commune: RoomName,
    /// The paths from the controller to the sources
    /* #[serde(with = "screeps::local::serde_position_packed")] */
    pub source_paths: Vec<Vec<Position>>,
    /// Not really sure what this is for
    pub cost: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// How long to abandon this remote for, generally due to unsustainable conflict-costs or invaders that can't be fought off
    pub abandon: Option<u32>,
}

impl RemoteRoomMemory {
    pub fn new(
        room_name: &RoomName,
        game_state: &mut GameState,
        cost: u32,
        source_paths: Vec<Vec<Position>>,
    ) -> Self {
        Self {
            commune: *room_name,
            source_paths,
            cost,
            abandon: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvaderCodeInfo {
    /// The level of the invader code
    pub level: u16,
    /// The tick at which the invader code will decay
    pub decay_by: u32,
}

#[derive(Serialize, Deserialize, Debug)]
/// Rooms claimed by allies
pub struct AllyRoomMemory {}

impl AllyRoomMemory {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Rooms claimed by enemies
pub struct EnemyRoomMemory {
    pub terminal: bool,
    pub stored_energy: u32,
    pub min_hits_to_breach: Option<u32>,
}

impl EnemyRoomMemory {
    pub fn new() -> Self {
        Self {
            terminal: false,
            stored_energy: 0,
            min_hits_to_breach: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Rooms with sources and minerals (In the base game all rooms with sources also have minerals)
pub struct HarvestableRoomMemory {
    /* #[serde(with = "screeps::local::serde_position_packed")] */
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Communes that have tried and failed, based on distance, to make this room a remote
    pub remote_blacklist: Option<HashSet<RoomName>>,
    pub source_positions: Vec<Position>,
    #[serde(with = "screeps::local::serde_position_packed")]
    pub mineral_pos: Position,
    pub mineral_type: constants::minerals::ResourceType,
}

impl HarvestableRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Result<Self, GeneralError> {
        let sources = room_ops::get_sources(room_name, game_state);
        let source_positions: Vec<Position> = sources.iter().map(|source| source.pos()).collect();

        let Some(room) = game_state.rooms.get(room_name) else {
            return Err(GeneralError::Fail);
        };

        // Mineral type

        let minerals = room.find(find::MINERALS, None);
        let Some(mineral) = minerals.first() else {
            return Err(GeneralError::Fail);
        };
        let mineral_type = mineral.mineral_type();
        let mineral_pos = mineral.pos();

        Ok(Self {
            source_positions,
            /// TODO
            mineral_pos,
            mineral_type,
            remote_blacklist: None,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Rooms that have portals
pub struct PortalRoomMemory {
    /// Portal positions
    pub portals: Vec<(RoomName, Position)>,
}

impl PortalRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Result<Self, GeneralError> {
        let Some(room) = game_state.rooms.get(room_name) else {
            return Err(GeneralError::Fail);
        };

        let portals = &room_ops::structures_by_type(room_name, game_state).portal;
        let portals_with_data = portals
            .iter()
            .map(|portal| {
                (
                    match portal.destination() {
                        PortalDestination::InterRoom(pos) => pos.room_name(),
                        PortalDestination::InterShard(destination) => destination.room(),
                    },
                    portal.pos(),
                )
            })
            .collect();

        Ok(Self {
            portals: portals_with_data,
        })
    }
}
