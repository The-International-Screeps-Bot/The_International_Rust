use screeps::Creep;

#[derive(Clone, Debug)]
pub struct NotMyCreeps {
    pub ally: Vec<Creep>,
    pub enemy: Vec<Creep>
}

impl NotMyCreeps {
    pub fn new() -> Self {
        NotMyCreeps {
            ally: Vec::new(),
            enemy: Vec::new(),
        }
    }
}

pub const MAX_REMOTE_ROOM_DISTANCE: u8 = 5;