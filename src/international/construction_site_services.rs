use screeps::{game, ConstructionSite};

use crate::{constants::general::{CONSTRUCTION_PROGRESS_AGE_MULTIPLIER, MIN_CONSTRUCTION_SITE_AGE}, memory::game_memory::GameMemory, utils::general::GeneralUtils};

pub struct ConstructionSiteServices;

impl ConstructionSiteServices {
    /// Register new construction sites, 
    /// remove keys for sites that no longer exist,
    /// delete and remove sites that are too old
    /// increment the age of remaining sites
    pub fn manage_sites(memory: &mut GameMemory) {
        let interval = 100;
        if !GeneralUtils::is_tick_interval(interval) {
            return
        }

        let construction_sites = game::construction_sites();

        for id in construction_sites.keys() {
            if memory.construction_sites.contains_key(id) {
                continue
            }

            memory.construction_sites.insert(*id, 0);
        }

        for (id, age) in &memory.construction_sites {
            let Some(site) = game::get_object_by_id_typed(id) else {

                memory.construction_sites.remove(&id);
                continue;
            };

            if *age > Self::max_site_age(&site) {
                site.remove();
                memory.construction_sites.remove(&id);
                continue;
            }

            memory.construction_sites.insert(id.clone(), age + 1 * interval);
        }
    }

    fn max_site_age(site: &ConstructionSite) -> u32 {
        MIN_CONSTRUCTION_SITE_AGE + site.progress() * CONSTRUCTION_PROGRESS_AGE_MULTIPLIER
    }
}