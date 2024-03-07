use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use screeps::{pathfinder::SearchGoal, Direction, Position};

use crate::{
    constants::general::{GeneralResult, DIRECTIONS},
    utils::general::GeneralUtils,
};

pub struct PathfinderOpts {
    pub cost_callback: fn(&Position) -> u8,
}

pub struct PathGoal {
    pos: Position,
    range: u32,
}
/// Position -> range map
pub type PathGoals = HashMap<Position, u32>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PathfinderOpenSetEntry {
    /// the position the entry is for
    pos: Position,
    /// g_score represents the cost of the best known path to get to this node
    g_score: u32,
    /// f_score represents the estimated total cost of a path through this node,
    /// adding the best known cost to the node (the g_score) to the heuristic's estimate of the
    /// cost to get from the node to the goal
    f_score: u32,
    /// direction this entry was opened from
    open_dir: Option<Direction>,
}

impl Ord for PathfinderOpenSetEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // we're using a max-heap but the behavior we want is min-heap, usual ordering is inverted
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for PathfinderOpenSetEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PathfinderOpenSetEntry {
    pub fn new(
        pos: Position,
        g_score: u32,
        goals_set: &HashSet<Position>,
        open_dir: Option<Direction>,
    ) -> Self {
        let heuristic_cost = PathfinderOps::get_heuristic_cost_to_closest_goal(pos, &goals_set);

        PathfinderOpenSetEntry {
            pos,
            g_score,
            f_score: g_score + heuristic_cost,
            open_dir,
        }
    }
}

pub struct PathfinderOps;

impl PathfinderOps {
    pub fn find_path(
        origin:   Position,
        goals: &PathGoals,
        opts: Option<PathfinderOpts>,
    ) -> Result<Vec<Position>, GeneralResult> {

        let mut open_set = BinaryHeap::new();
        let mut visited = HashMap::new();

        let goals_set: HashSet<Position> = goals.keys().copied().collect();

        open_set.push(PathfinderOpenSetEntry::new(origin, 0, &goals_set, None));
        visited.insert(origin, None);

        while let Some(open_set_entry) = open_set.pop() {
            for direction in DIRECTIONS {
                if Some(-direction) == open_set_entry.open_dir {
                    continue;
                }
                let Ok(adj_pos) = open_set_entry.pos.checked_add((direction).into()) else {
                    continue;
                };

                if visited.contains_key(&adj_pos) {
                    continue;
                }

                visited.insert(adj_pos, Some(direction));

                if goals_set.contains(&adj_pos) {
                    let path = PathfinderOps::resolve_completed_path(adj_pos, &visited);
                    return Ok(path.into_iter().collect());
                }

                let mut adj_traverse_cost: u8 = 0;
                
                if let Some(opts) = &opts {
                    adj_traverse_cost = (opts.cost_callback)(&adj_pos);
                };

                if adj_traverse_cost >= u8::MAX {
                    continue;
                }

                open_set.push(PathfinderOpenSetEntry::new(
                    adj_pos,
                    open_set_entry.g_score + adj_traverse_cost as u32,
                    &goals_set,
                    Some(direction),
                ));
            }
        }

        Err(GeneralResult::Fail)
    }
    /// Find cost as the lowest manhattan distance to any goal
    fn get_heuristic_cost_to_closest_goal(pos: Position, goals: &HashSet<Position>) -> u32 {
        let mut lowest_cost = u32::MAX;
        for goal in goals {
            let cost =
                pos.world_x().abs_diff(goal.world_x()) + pos.world_y().abs_diff(goal.world_y());
            if cost < lowest_cost {
                lowest_cost = cost;
            }
        }
        lowest_cost
    }

    /// navigate backwards across our map of where tiles came from to construct a path
    fn resolve_completed_path(
        pos: Position,
        visited: &HashMap<Position, Option<Direction>>,
    ) -> HashSet<Position> {
        let mut path = HashSet::new();
        path.insert(pos);

        let mut cursor_pos = pos;

        while let Some(optional_search_direction) = visited.get(&cursor_pos) {
            match optional_search_direction {
                Some(search_dir) => {
                    if let Ok(next_pos) = cursor_pos.checked_add((-*search_dir).into()) {
                        path.insert(next_pos);
                        cursor_pos = next_pos;
                    }
                }
                None => break,
            }
        }

        path
    }
}
