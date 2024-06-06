use screeps::Room;

pub struct MyRoom(Room);

impl MyRoom {
    pub fn new(room: &Room) -> Self {
        Self(room.clone())
    }

    #[inline]
    pub fn inner(&self) -> &Room {
        &self.0
    }
}