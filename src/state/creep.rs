use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CreepsState {
    pub costs: HashMap<String, u32>,
}

impl CreepsState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
