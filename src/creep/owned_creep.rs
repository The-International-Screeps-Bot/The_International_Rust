use screeps::Creep;

#[derive(Clone, Debug)]
pub struct OwnedCreep(Creep);

impl OwnedCreep {
    pub fn new(creep: &Creep) -> Result<Self, ()> {
        if creep.my() {
            Ok(Self(creep.clone()))
        } else {
            Err(())
        }
    }

    pub fn inner(&self) -> &Creep {
        &self.0
    }
}
