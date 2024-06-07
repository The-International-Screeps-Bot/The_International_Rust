use std::collections::{HashMap, HashSet};

use screeps::{game, ConstructionSite, MaybeHasId, ObjectId};

use crate::{
    constants::general::{CONSTRUCTION_PROGRESS_AGE_MULTIPLIER, MIN_CONSTRUCTION_SITE_AGE},
    memory::game_memory::GameMemory,
    utils::general::GeneralUtils,
};

/// Register new construction sites,
/// remove keys for sites that no longer exist,
/// delete and remove sites that are too old
/// increment the age of remaining sites
#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn manage_sites(memory: &mut GameMemory) {
    // only run the following logic every so often

    let interval = 100;
    if !GeneralUtils::is_tick_interval(interval) {
        return;
    }

    /// Construction site ids that are known to exist
    let mut construction_sites: HashMap<ObjectId<ConstructionSite>, ConstructionSite> =
        HashMap::new();
    let js_construction_sites = game::construction_sites();

    // Move the values from a js hashmap to the rust hashmap

    for site in js_construction_sites.values() {
        let Some(id) = site.try_id() else {
            continue;
        };
        construction_sites.insert(id, site);
    }

    // Remove caches for construction sites that no longer exist,
    // delete and remove caches for construction sites that are too old relative to their progress

    memory.construction_sites.retain(|id, age| {
        let Some(site) = construction_sites.get(id) else {
            return false;
        };

        if *age > max_site_age(site.progress()) {
            site.remove();
            return false;
        }

        true
    });

    // Increment the age of cached construction sites

    for (id, age) in &mut memory.construction_sites {
        // times inveral because we only run the code every interval, but we cant to track how many ticks have passed
        *age += interval;
    }
}

fn max_site_age(site_progress: u32) -> u32 {
    MIN_CONSTRUCTION_SITE_AGE + site_progress * CONSTRUCTION_PROGRESS_AGE_MULTIPLIER
}
