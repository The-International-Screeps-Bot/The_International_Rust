use screeps::{Creep, HasPosition};

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct OwnedCreep(Creep);

impl OwnedCreep {
    pub fn new(creep: &Creep) -> Result<Self, ()> {
        if creep.my() {
            Ok(Self(creep.clone()))
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn inner(&self) -> &Creep {
        &self.0
    }
}
