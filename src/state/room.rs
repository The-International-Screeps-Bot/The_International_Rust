use std::collections::HashMap;

use screeps::{find, Room, RoomName, StructureObject, StructureProperties, StructureType};

use crate::constants::structure::OrganizedStructures;

use super::game::GameState;

#[derive(Debug)]
pub struct RoomState {
    pub name: RoomName,
    pub structures: Option<OrganizedStructures>,
}

impl RoomState {
    pub fn new(room_name: RoomName) -> Self {
        Self {
            name: room_name,
            structures: Some(OrganizedStructures::default()),
        }
    }

    // pub fn structures(
    //     &mut self,
    //     room: &Room,
    // ) -> Option<&HashMap<StructureType, Vec<StructureObject>>> {
    //     match &self.structures {
    //         Some(organized_structures) => Some(organized_structures),
    //         None => {
    //             Some({
    //                 let mut new_organized_structures = HashMap::new();
    //                 for structure in room.find(find::STRUCTURES, None) {
    //                     let structure_type = structure.structure_type();
    //                     new_organized_structures
    //                         .entry(structure_type)
    //                         .or_insert(Vec::new())
    //                         .push(structure);
    //                 }

    //                 new_organized_structures
    //             })
    //         }
    //     }

    //     // if let Some(organized_structures) = self.structures {
    //     //     return &Some(organized_structures);
    //     // }

    //     // let mut new_organized_structures = HashMap::new();
    //     // for structure in room.find(find::STRUCTURES, None) {
    //     //     let structure_type = structure.structure_type();
    //     //     new_organized_structures
    //     //         .entry(structure_type)
    //     //         .or_insert(Vec::new())
    //     //         .push(structure);
    //     // }

    //     // &self.structures
    // }
}

pub struct CommunePlanner {
    grid_map: [u8; 2500],
    terrain_map: [u8; 2500],
    road_map: [u8; 2500],
    plan_map: [u8; 2500],
}

pub struct RemotePlanner {}

pub struct RoomStateOps;

impl RoomStateOps {
    pub fn update_state(state: &mut RoomState) {
        if let Some(organized_structures) = state.structures.as_mut() {
            organized_structures.clear();
        }
    }
}
