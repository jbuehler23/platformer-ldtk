// In a new abilities.rs file
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Abilities {
    can_block: bool,
}

impl Abilities {
    pub fn unlock_block(&mut self) {
        self.can_block = true;
    }

    pub fn can_block(&self) -> bool {
        self.can_block
    }
}