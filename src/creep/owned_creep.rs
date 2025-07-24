use core::ops::Deref;

use screeps::{game, Creep};

/// calls the passed function with a list of all creeps that the player owns.
/// this is preferable to repeatedly calling `game::creeps` because it prevents multiple redundant calls.
pub fn with_owned_creeps<F: for<'a, 'creep> FnOnce(&'a [OwnedCreep<'creep>])>(f: F) {
    let creeps = game::creeps().values().collect::<Vec<_>>();
    let owned_creeps = creeps.iter().map(|c| OwnedCreep(c)).collect::<Vec<_>>();
    f(owned_creeps.as_slice());
}

#[derive(Debug, Clone, Copy)]
pub struct OwnedCreep<'a>(&'a Creep);

// allows access to the internal Creep methods without needing to duplicate
impl<'a> Deref for OwnedCreep<'a> {
    type Target = Creep;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
