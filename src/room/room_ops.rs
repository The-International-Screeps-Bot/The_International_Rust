use std::collections::HashMap;

use enum_map::{enum_map, EnumMap};
use screeps::{
    find,
    game::{self, map::RoomStatus},
    look, structure, Creep, HasPosition, ObjectId, Position, Room, RoomName, SharedCreepProperties,
    Source, Structure, StructureContainer, StructureController, StructureExtension,
    StructureExtractor, StructureFactory, StructureInvaderCore, StructureKeeperLair, StructureLink,
    StructureNuker, StructureObject, StructureObserver, StructurePowerBank, StructurePowerSpawn,
    StructureProperties, StructureRampart, StructureRoad, StructureSpawn, StructureStorage,
    StructureTerminal, StructureTower, StructureType, StructureWall, Terrain,
};

use crate::{
    constants::structure::{OldOrganizedStructures, OrganizedStructures, SpawnsByActivity},
    memory::{game_memory::GameMemory, room_memory::RoomMemory},
    settings::Settings,
    state::{
        game::GameState,
        market::MarketState,
        room::{self, RoomState},
    },
    utils::general::GeneralUtils,
    GAME_STATE,
};

pub struct RoomOps;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl RoomOps {
    /// Acquires and caches structures in the room based on their structure type
    /*     pub fn structures<'room, 'state>(
        room_name: &'room RoomName,
        game_state: &'state mut GameState,
    ) -> &'state OldOrganizedStructures {

        let room = game_state.rooms.get(room_name).unwrap();
        let room_state = game_state.room_states.get_mut(room_name).unwrap();

        room_state.structures.get_or_insert_with(|| {
            let mut new_organized_structures = HashMap::new();
            for structure in room.find(find::STRUCTURES, None) {
                let structure_type = structure.structure_type();
                new_organized_structures
                    .entry(structure_type)
                    .or_insert(Vec::new())
                    .push(structure);
            }
            new_organized_structures
        })
    } */

    #[inline]
    pub fn structures<'room, 'state>(
        room_name: &'room RoomName,
        game_state: &'state mut GameState,
    ) -> &'state OrganizedStructures {
        let room = game_state.rooms.get(room_name).unwrap();
        let room_state = game_state.room_states.get_mut(room_name).unwrap();

        let organized_structures = room_state.structures.get_or_insert_with(|| {
            let mut new_organized_structures = OrganizedStructures {
                ..Default::default()
            };
            for structure in room.find(find::STRUCTURES, None) {
                match structure.structure_type() {
                    StructureType::Spawn => {
                        new_organized_structures
                            .spawn
                            .push(TryInto::<StructureSpawn>::try_into(structure).unwrap());
                    }
                    StructureType::Extension => {
                        new_organized_structures
                            .extension
                            .push(TryInto::<StructureExtension>::try_into(structure).unwrap());
                    }
                    StructureType::Road => {
                        new_organized_structures
                            .road
                            .push(TryInto::<StructureRoad>::try_into(structure).unwrap());
                    }
                    StructureType::Wall => {
                        new_organized_structures
                            .wall
                            .push(TryInto::<StructureWall>::try_into(structure).unwrap());
                    }
                    StructureType::Rampart => {
                        new_organized_structures
                            .rampart
                            .push(TryInto::<StructureRampart>::try_into(structure).unwrap());
                    }
                    StructureType::Container => {
                        new_organized_structures
                            .container
                            .push(TryInto::<StructureContainer>::try_into(structure).unwrap());
                    }
                    StructureType::Link => {
                        new_organized_structures
                            .link
                            .push(TryInto::<StructureLink>::try_into(structure).unwrap());
                    }
                    StructureType::KeeperLair => {
                        new_organized_structures
                            .keeper_lair
                            .push(TryInto::<StructureKeeperLair>::try_into(structure).unwrap());
                    }
                    StructureType::PowerBank => {
                        new_organized_structures
                            .power_bank
                            .push(TryInto::<StructurePowerBank>::try_into(structure).unwrap());
                    }
                    StructureType::Tower => {
                        new_organized_structures
                            .tower
                            .push(TryInto::<StructureTower>::try_into(structure).unwrap());
                    }
                    StructureType::InvaderCore => {
                        new_organized_structures
                            .invader_core
                            .push(TryInto::<StructureInvaderCore>::try_into(structure).unwrap());
                    }
                    _ => {}
                }
            }
            new_organized_structures
        });
        organized_structures
    }

    #[inline]
    pub fn storage<'state>(
        room_name: &RoomName,
        game_state: &'state mut GameState,
    ) -> &'state Option<StructureStorage> {
        let room = game_state.rooms.get(room_name).unwrap();

        let room_state = game_state.room_states.get_mut(room_name).unwrap();

        room_state.storage = room.storage();
        &room_state.storage
    }

    #[inline]
    pub fn controller<'state>(
        room_name: &RoomName,
        game_state: &'state mut GameState,
    ) -> &'state Option<StructureController> {
        let room = game_state.rooms.get(room_name).unwrap();

        let room_state = game_state.room_states.get_mut(room_name).unwrap();

        room_state.controller = room.controller();
        &room_state.controller
    }

    #[inline]
    pub fn terminal<'state>(
        room_name: &RoomName,
        game_state: &'state mut GameState,
    ) -> &'state Option<StructureTerminal> {
        let room = game_state.rooms.get(room_name).unwrap();

        let room_state = game_state.room_states.get_mut(room_name).unwrap();

        room_state.terminal = room.terminal();
        &room_state.terminal
    }

    // pub fn spawns_by_activity<'state>(
    //     room_name: &RoomName,
    //     game_state: &'state mut GameState,
    // ) -> &'state Option<SpawnsByActivity<'state>> {

    //     let structures = Self::structures(room_name, game_state);

    //     let Some(commune_state) = game_state.commune_states.get_mut(room_name) else {
    //         return &None
    //     };

    //     let spawns_by_activity = commune_state.spawns_by_activity.get_or_insert_with(|| SpawnsByActivity::new());

    //     for spawn in &structures.spawn {
    //         match spawn.spawning() {
    //             Some(spawning) => spawns_by_activity.active.push(&spawn),
    //             _ => spawns_by_activity.inactive.push(&spawn),
    //         }
    //     }

    //     &commune_state.spawns_by_activity
    // }

    // pub fn spawns_by_activity<'state>(
    //     room_name: &RoomName,
    //     game_state: &'state mut GameState<'state>,
    // ) -> &'state Option<SpawnsByActivity<'state>> {
    //     let mut spawns_by_activity = {
    //         let spawns_by_activity = SpawnsByActivity::new();

    //         let structures = Self::structures(room_name, game_state);

    //         for spawn in &structures.spawn {
    //             match spawn.spawning() {
    //                 Some(spawning) => spawns_by_activity.active.push(&spawn),
    //                 _ => spawns_by_activity.inactive.push(&spawn),
    //             }
    //         }

    //         spawns_by_activity
    //     };

    //     let Some(commune_state) = game_state.commune_states.get_mut(room_name) else {
    //         return &None;
    //     };

    //     commune_state.spawns_by_activity = Some(spawns_by_activity);
    //     &commune_state.spawns_by_activity
    // }

    // pub fn get_sources(room: &Room, room_data: &mut RoomData) -> Option<Vec<Source>> {

    //     let mut sources = &room_data.sources;
    //     if let Some(sources) = sources {
    //         return Some(sources.clone())
    //     };

    //     let new_sources: Vec<Source> = room.find(find::SOURCES, None);
    //     room_data.sources = Some(new_sources.clone());
    //     Some(new_sources)
    // }

    // /// Gets creeps that we don't own, seperated into those that are enemies and those that are allies
    // pub fn not_my_creeps(room: &Room, room_data: &mut RoomData, settings: &Settings) -> Option<NotMyCreeps> {

    //     let mut not_my_creeps = &room_data.not_my_creeps;
    //     if let Some(enemy_creeps) = not_my_creeps {
    //         return Some(enemy_creeps.clone())
    //     }

    //     let mut new_not_my_creeps: NotMyCreeps = NotMyCreeps::new();

    //     let unorganized_not_my_creeps: Vec<Creep> = room.find(find::HOSTILE_CREEPS, None);
    //     for creep in unorganized_not_my_creeps {
    //         if settings.allies.contains(&creep.name()) {
    //             new_not_my_creeps.ally.push(creep);
    //             continue;
    //         }

    //         new_not_my_creeps.enemy.push(creep);
    //     }

    //     room_data.not_my_creeps = Some(new_not_my_creeps.clone());
    //     Some(new_not_my_creeps)
    // }

    // pub fn harvest_positions(room: &Room, room_data: &mut RoomData) -> Option<Vec<Position>> {

    //     let harvest_positions = &room_data.harvest_positions;
    //     if let Some(harvest_positions) = harvest_positions {
    //         return Some(harvest_positions.clone())
    //     }

    //     let sources = RoomOps::get_sources(room, room_data);
    //     let Some(sources) = sources else {
    //         return None;
    //     };

    //     let new_harvest_positions: Vec<Position> = Vec::new();

    //     for source in sources {
    //         GeneralUtils::for_adjacent_positions(source.pos(), &|adjacent_pos| {
    //             let terrain = room.look_for_at(look::TERRAIN, adjacent_pos);
    //             terrain.contains(&Terrain::Wall);

    //             ();
    //         })
    //     }

    //     room_data.harvest_positions = Some(new_harvest_positions.clone());
    //     Some(new_harvest_positions)
    // }

    pub fn room_status(room_name: &RoomName, memory: &mut GameMemory) -> Option<RoomStatus> {
        let Some(room_memory) = memory.rooms.get_mut(room_name) else {
            return None;
        };

        if let Some(status) = room_memory.status {
            return Some(status);
        }

        let status_result = game::map::get_room_status(*room_name).unwrap();
        let new_status = status_result.status();

        room_memory.status = Some(new_status.clone());
        Some(new_status)
    }

    pub fn move_creeps(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {
        let Some(room) = game_state.rooms.get_mut(room_name) else {
            return;
        };

        let Some(room_state) = game_state.room_states.get_mut(room_name) else {
            return;
        };

        let Some(room_memory) = memory.rooms.get_mut(room_name) else {
            return;
        };

        Self::move_creep(room_name, game_state, memory);
    }

    pub fn move_creep(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {}
}
