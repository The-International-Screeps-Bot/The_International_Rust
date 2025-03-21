use std::{
    collections::{HashMap, HashSet},
    io::Error,
    str::FromStr,
};

use enum_map::{enum_map, EnumMap};
use log::{debug, warn};
use screeps::{
    find,
    game::{self, map::RoomStatus},
    look, structure, ConstructionSite, Creep, HasPosition, LocalRoomTerrain, ObjectId,
    OwnedStructureProperties, Position, Room, RoomCoordinate, RoomName, RoomTerrain, RoomXY,
    SharedCreepProperties, Source, Structure, StructureContainer, StructureController,
    StructureExtension, StructureExtractor, StructureFactory, StructureInvaderCore,
    StructureKeeperLair, StructureLink, StructureNuker, StructureObject, StructureObserver,
    StructurePowerBank, StructurePowerSpawn, StructureProperties, StructureRampart, StructureRoad,
    StructureSpawn, StructureStorage, StructureTerminal, StructureTower, StructureType,
    StructureWall, Terrain, CREEP_RANGED_ACTION_RANGE,
};
use screeps_utils::sparse_cost_matrix::SparseCostMatrix;

use crate::{
    constants::{
        general::{FlowResult, GeneralError},
        move_costs::{DEFAULT_SWAMP_COST, DEFAULT_WALL_COST, MAX_COST},
        room::{NotMyCreeps, MAX_REMOTE_ROOM_DISTANCE},
        structure::{
            OldOrganizedStructures, OrganizedStructures, SpawnsByActivity, IMPASSIBLE_STRUCTURES,
        },
    },
    memory::{
        game_memory::GameMemory,
        room_memory::{
            AllyRoomMemory, CenterRoomMemory, EnemyRoomMemory, HighwayRoomMemory,
            IntersectionRoomMemory, KeeperRoomMemory, NeutralRoomMemory, RemoteRoomMemory,
            RoomMemory, StaticRo,omType
        },
    },
    pathfinding::{pathfinding_services::PathfindingOpts, portal_router, room_costs, route_costs},
    settings::Settings,
    state::{
        game::GameState,
        market::MarketState,
        room::{self, NotMyConstructionSites, RoomState},
    },
    utils::{
        self,
        general::{for_adjacent_positions, GeneralUtils},
        pos::{for_positions_in_range_in_room, get_positions_in_range_in_room},
    },
    GAME_STATE,
};

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

pub fn enemy_threat_positions<'state>(
    room_name: &RoomName,
    game_state: &'state mut GameState,
    memory: &mut GameMemory,
) -> SparseCostMatrix {
    {
        let room_state = game_state.room_states.get(room_name).unwrap();

        if let Some(enemy_threat_positions) = &room_state.enemy_threat_positions {
            return enemy_threat_positions.clone();
        }
    }

    let mut threat_positions = SparseCostMatrix::new();
    let enemy_creeps = not_my_creeps(room_name, game_state, memory).enemy.clone();

    for creep in enemy_creeps {
        let positions =
            get_positions_in_range_in_room(&creep.pos(), CREEP_RANGED_ACTION_RANGE.into());

        for pos in positions {
            threat_positions.set(pos.xy(), MAX_COST);
        }
    }

    let room_state = game_state.room_states.get_mut(room_name).unwrap();

    room_state.enemy_threat_positions = Some(threat_positions.clone());
    threat_positions
}

pub fn structures<'state>(
    room_name: &RoomName,
    game_state: &'state mut GameState,
) -> &'state Vec<StructureObject> {
    let room_state = game_state.room_states.get_mut(room_name).unwrap();

    let structures = room_state.structures.get_or_insert_with(
        (|| {
            let room = game_state.rooms.get(room_name).unwrap();
            room.find(find::STRUCTURES, None)
        }),
    );

    structures
}

pub fn my_construction_sites<'state>(
    room_name: &RoomName,
    game_state: &'state mut GameState,
) -> &'state Vec<ConstructionSite> {
    let room_state = game_state.room_states.get_mut(room_name).unwrap();

    let c_sites = room_state.my_construction_sites.get_or_insert_with(
        (|| {
            let room = game_state.rooms.get(room_name).unwrap();
            room.find(find::MY_CONSTRUCTION_SITES, None)
        }),
    );

    c_sites
}

pub fn not_my_construction_sites<'state>(
    room_name: &RoomName,
    game_state: &'state mut GameState,
    memory: &GameMemory,
) -> &'state NotMyConstructionSites {
    let room_state = game_state.room_states.get_mut(room_name).unwrap();

    let c_sites = room_state.not_my_construction_sites.get_or_insert_with(
        (|| {
            let mut not_my_construction_sites = NotMyConstructionSites::new();

            let room = game_state.rooms.get(room_name).unwrap();
            let hostile_c_sites = room.find(find::HOSTILE_CONSTRUCTION_SITES, None);

            for c_site in hostile_c_sites {
                if memory.allies.contains_key(&c_site.owner().username()) {
                    not_my_construction_sites.ally.push(c_site);
                    continue;
                }

                not_my_construction_sites.enemy.push(c_site);
            }

            not_my_construction_sites
        }),
    );

    c_sites
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn structures_by_type<'state>(
    room_name: &RoomName,
    game_state: &'state mut GameState,
) -> &'state OrganizedStructures {
    let room_state = game_state.room_states.get_mut(room_name).unwrap();

    let organized_structures = room_state.structures_by_type.get_or_insert_with(|| {
        let mut new_organized_structures = OrganizedStructures {
            ..Default::default()
        };

        let room = game_state.rooms.get(room_name).unwrap();

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

// /// Gets creeps that we don't own, seperated into those that are enemies and those that are allies
pub fn not_my_creeps(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
) -> NotMyCreeps {
    let room_data = game_state.room_states.get_mut(room_name).unwrap();

    let mut not_my_creeps = &room_data.not_my_creeps;
    if let Some(enemy_creeps) = not_my_creeps {
        return enemy_creeps.clone();
    }

    let mut new_not_my_creeps: NotMyCreeps = NotMyCreeps::new();

    let room = game_state.rooms.get(room_name).unwrap();

    let unorganized_not_my_creeps: Vec<Creep> = room.find(find::HOSTILE_CREEPS, None);
    for creep in unorganized_not_my_creeps {
        if memory.allies.contains_key(&creep.name()) {
            new_not_my_creeps.ally.push(creep);
            continue;
        }

        new_not_my_creeps.enemy.push(creep);
    }

    room_data.not_my_creeps = Some(new_not_my_creeps.clone());
    new_not_my_creeps
}

pub fn get_sources(room_name: &RoomName, game_state: &mut GameState) -> Vec<Source> {
    let room_data = game_state.room_states.get_mut(room_name).unwrap();

    let mut sources = &room_data.sources;
    if let Some(sources) = sources {
        return sources.clone();
    };

    let room = game_state.rooms.get(room_name).unwrap();

    let new_sources: Vec<Source> = room.find(find::SOURCES, None);
    room_data.sources = Some(new_sources.clone());
    new_sources
}

pub fn commune_sources(room_name: &RoomName, game_state: &mut GameState) -> Vec<Source> {
    get_sources(room_name, game_state)
}

pub fn harvest_positions(
    room_name: &RoomName,
    game_state: &mut GameState,
) -> Option<Vec<Position>> {
    {
        let room_data = game_state.room_states.get(room_name).unwrap();

        let harvest_positions = &room_data.harvest_positions;
        if let Some(harvest_positions) = harvest_positions {
            return Some(harvest_positions.clone());
        }
    }

    let sources = get_sources(room_name, game_state);

    let new_harvest_positions: Vec<Position> = Vec::new();
    let room = game_state.rooms.get(room_name).unwrap();

    for source in sources {
        utils::general::for_adjacent_positions(source.pos(), &|adjacent_pos| {
            let terrain = room.look_for_at(look::TERRAIN, adjacent_pos);
            terrain.contains(&Terrain::Wall);
        })
    }

    let room_data = game_state.room_states.get_mut(&room.name()).unwrap();

    room_data.harvest_positions = Some(new_harvest_positions.clone());
    Some(new_harvest_positions)
}

pub fn room_type(room_name: &RoomName, memory: &mut GameMemory) {}

pub fn room_status(room_name: &RoomName, game_state: &mut GameState) -> RoomStatus {
    let room_state = game_state.room_states.get_mut(room_name).unwrap();

    if let Some(status) = room_state.status {
        return status;
    }

    let status_result = game::map::get_room_status(*room_name).unwrap();
    let new_status = status_result.status();

    room_state.status = Some(new_status);
    new_status
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
}

/* pub fn test_state(room: &Room, room_state: &mut RoomState, game_state: &mut GameState, memory: &mut GameMemory) {

}

pub fn test_state_name(room_name: &RoomName, room_state: &mut RoomState, game_state: &mut GameState, memory: &mut GameMemory) {

}

pub fn test_name(room_name: &RoomName, room: &Room, game_state: &mut GameState, memory: &mut GameMemory) {

} */

pub fn try_add_remote(
    room_name: &RoomName,
    scouting_room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> FlowResult {
    // If the room is already a remote

    if let Some(remote_memory) = memory.remotes.get(room_name) {
        // If the remote already has the scouting room as its commune
        if &remote_memory.commune == scouting_room_name {
            return FlowResult::Continue;
        }

        // If the remote no longer has a valid commune, remove it's remote_memory
        if !game_state.communes.contains(&remote_memory.commune) {
            memory.remotes.remove(room_name);

            memory
                .neutral
                .insert(*room_name, NeutralRoomMemory::new(room_name, game_state));
        }
    }

    // Check if the commune is blacklisted
    {
        let neutral_memory = memory.neutral.get(room_name).unwrap();
        if neutral_memory.remote_blacklist.contains(room_name) {
            return FlowResult::Continue;
        }
    }

    // Check the linear distance
    let distance = range(room_name, scouting_room_name);
    if distance > MAX_REMOTE_ROOM_DISTANCE.into() {
        debug!("Room {} too far from {}", room_name, scouting_room_name);
        return FlowResult::Continue;
    }

    // Check the route distance

    let mut goals = HashSet::new();
    goals.insert(*scouting_room_name);

    let route = portal_router::find_route(*room_name, goals, &PathfindingOpts::new(), &memory);

    let Ok(route) = route else {
        warn!("Unable to find route for room {}", room_name);
        return FlowResult::Continue;
    };

    if route.len() as u8 > MAX_REMOTE_ROOM_DISTANCE {
        debug!("Route too long for room {}", room_name);

        let neutral_memory = memory.neutral.get_mut(room_name).unwrap();
        neutral_memory.remote_blacklist.insert(*room_name);

        return FlowResult::Continue;
    }

    // Cost and source info

    let mut cost: u32 = 0;

    let sources = get_sources(room_name, game_state);
    let mut source_paths: Vec<Vec<Position>> = Vec::new();

    for (i, source) in sources.into_iter().enumerate() {
        // Calculate path results

        let path: Vec<Position> = Vec::new();
        cost += path.len() as u32;

        source_paths.push(path);
    }

    if let Some(remote_memory) = memory.remotes.get(room_name) {
        if cost >= remote_memory.cost {
            return FlowResult::Continue;
        }

        memory.remotes.remove(room_name);

        memory
            .neutral
            .insert(*room_name, NeutralRoomMemory::new(room_name, game_state));
    }

    let mut remote_memory = RemoteRoomMemory::new(room_name, game_state, cost, source_paths);

    memory.remotes.insert(*room_name, remote_memory);

    FlowResult::Stop
}

pub fn terrain(room_name: &RoomName, game_state: &mut GameState) -> LocalRoomTerrain {
    {
        let room_state = game_state.room_states.get(room_name).unwrap();

        if let Some(terrain) = &room_state.terrain {
            return terrain.clone();
        }
    }

    let room_state = game_state.room_states.get_mut(room_name).unwrap();

    let js_terrain = game::map::get_room_terrain(*room_name).unwrap();
    let terrain = LocalRoomTerrain::from(js_terrain);

    room_state.terrain = Some(terrain.clone());
    terrain
}

pub fn default_move_costs(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
) -> SparseCostMatrix {
    {
        let room_state = game_state.room_states.get(room_name).unwrap();

        if let Some(default_move_ops) = &room_state.default_move_ops {
            return default_move_ops.clone();
        }
    }

    let mut default_move_ops = SparseCostMatrix::new();

    // Avoid terrain

    let terrain = terrain(room_name, game_state);
    for x in 0..50 {
        for y in 0..50 {
            let room_xy = RoomXY {
                x: RoomCoordinate::new(x).unwrap(),
                y: RoomCoordinate::new(y).unwrap(),
            };

            let terrain_type = terrain.get_xy(room_xy);
            match terrain_type {
                Terrain::Swamp => {
                    default_move_ops.set(room_xy, DEFAULT_SWAMP_COST);
                }
                Terrain::Wall => {
                    default_move_ops.set(room_xy, DEFAULT_WALL_COST);
                }
                _ => {}
            }
        }
    }

    // Avoid impassible structures

    let structures = structures(room_name, game_state);
    for structure in structures {
        if !IMPASSIBLE_STRUCTURES.contains(&structure.structure_type()) {
            continue;
        }

        default_move_ops.set(structure.pos().xy(), 255);
    }

    // Avoid construction sites we own that are impassible

    let my_construction_sites = my_construction_sites(room_name, game_state);
    for construction_site in my_construction_sites {
        if !IMPASSIBLE_STRUCTURES.contains(&construction_site.structure_type()) {
            continue;
        }

        default_move_ops.set(construction_site.pos().xy(), 255);
    }

    // Avoid all ally construction sites

    let consturction_sites = &not_my_construction_sites(room_name, game_state, memory).ally;
    for construction_site in consturction_sites {
        default_move_ops.set(construction_site.pos().xy(), 255);
    }

    let room_state = game_state.room_states.get_mut(room_name).unwrap();
    room_state.default_move_ops = Some(default_move_ops.clone());
    default_move_ops
}

pub fn try_scout_room(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {
    if let Some(room_memory) = memory.rooms.get(room_name) {
        return;
    };

    // Otherwise the room has no memory

    let mut room_memory = RoomMemory::new(game_state);
    room_memory.room_type = find_room_type(room_name, game_state, memory);

    match room_memory.room_type {
        StaticRoomType::Ally => {
            let ally_memory = AllyRoomMemory::new();
            memory.ally.insert(*room_name, ally_memory);
        }
        StaticRoomType::Enemy => {
            let enemy_memory = EnemyRoomMemory::new();
            memory.enemy.insert(*room_name, enemy_memory);
        }
        StaticRoomType::Center => {
            let center_memory = CenterRoomMemory::new();
            memory.center.insert(*room_name, center_memory);
        }
        StaticRoomType::Highway => {
            let highway_memory = HighwayRoomMemory::new();
            memory.highway.insert(*room_name, highway_memory);
        }
        StaticRoomType::Keeper => {
            let keeper_memory = KeeperRoomMemory::new(room_name, game_state);
            memory.keeper.insert(*room_name, keeper_memory);
        }
        StaticRoomType::Intersection => {
            let intersection_memory = IntersectionRoomMemory::new();
            memory.intersection.insert(*room_name, intersection_memory);
        }
        _ => {}
    }

    memory.rooms.insert(*room_name, room_memory);
}

pub fn find_room_type(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> StaticRoomType {
    // Stop if we already have a room type
    if let Ok(room_type) = find_static_room_type(room_name) {
        return room_type;
    };

    let controller = controller(room_name, game_state);
    let Some(controller) = controller else {
        return StaticRoomType::Neutral;
    };

    let Some(owner) = controller.owner() else {
        return StaticRoomType::Neutral;
    };
    let owner_name = owner.username();

    if owner_name == memory.me {
        return StaticRoomType::Commune;
    }
    if memory.allies.contains_key(&owner_name) {
        return StaticRoomType::Ally;
    }

    StaticRoomType::Enemy
}

/// Attempt to deduce a static room type using some cheap math
pub fn find_static_room_type(room_name: &RoomName) -> Result<StaticRoomType, GeneralError> {
    let room_x = room_name.x_coord();
    let room_y = room_name.y_coord();

    let ew = room_x % 10;
    let ns = room_y % 10;

    if ew == 0 && ns == 0 {
        return Ok(StaticRoomType::Intersection);
    }
    if ew == 0 || ns == 0 {
        return Ok(StaticRoomType::Highway);
    }
    if room_x % 5 == 0 && room_y % 5 == 0 {
        return Ok(StaticRoomType::Center);
    }
    if (5 - ew).abs() <= 1 && (5 - ns).abs() <= 1 {
        return Ok(StaticRoomType::Keeper);
    }

    Err(GeneralError::Fail)
}

pub fn range(room_name1: &RoomName, room_name2: &RoomName) -> u32 {
    let x1 = room_name1.x_coord();
    let y1 = room_name1.y_coord();
    let x2 = room_name2.x_coord();
    let y2 = room_name2.y_coord();

    utils::general::xy_range(x1, y1, x2, y2)
}
