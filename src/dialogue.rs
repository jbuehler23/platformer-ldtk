use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct Dialogue {
    pub text: String,
}

impl Dialogue {
    pub fn from_field(entity_instance: &EntityInstance) -> Dialogue {
        Dialogue {
            text: entity_instance
                .get_string_field("dialogue")
                .expect("expected entity to have non-nullable dialogue String field")
                .clone()
        }
    }
}