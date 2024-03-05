use std::collections::HashMap;

#[derive(Default)]
pub struct CreepState {

}

impl CreepState {
    pub fn new() -> Self {
        CreepState {  }
    }
}

pub type CreepsState = HashMap<String, CreepState>;