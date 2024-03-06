use std::collections::HashMap;

use screeps::{
    find,
    game::{self, map::RoomStatus},
    look, Creep, HasPosition, ObjectId, Position, Room, RoomName, SharedCreepProperties, Source,
    Structure, StructureProperties, StructureType, Terrain,
};

use crate::{memory::game_memory::GameMemory, settings::Settings, utils::general::GeneralUtils};

pub struct RoomOps;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl RoomOps {
    // pub fn get_structures(room: &Room, room_data: &mut RoomData) -> Option<OrganizedStructures> {

    //     let mut organized_structures = &room_data.structures;
    //     if let Some(organized_structures) = organized_structures {
    //         return Some(organized_structures.clone())
    //     }

    //     let mut new_organized_structures: OrganizedStructures = HashMap::new();
    //     for structure in room.find(find::STRUCTURES, None) {
    //         let structure_type = structure.structure_type();
    //         new_organized_structures
    //             .entry(structure_type)
    //             .or_insert(Vec::new())
    //             .push(structure);
    //     }

    //     room_data.structures = Some(new_organized_structures.clone());
    //     Some(new_organized_structures)
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
}
