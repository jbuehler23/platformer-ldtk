use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{colliders::{spawn_npc_sensors, ColliderBundle, NPCColliderBundle}, dialogue::Dialogue, player::Player};

#[derive(Default, Component)]
pub struct NPC;

/// The main [LdtkEntity] for this example.
#[derive(Default, Bundle, LdtkEntity)]
pub struct NPCBundle {
    pub npc: NPC,
    #[with(name_from_field)]
    name: Name,
    #[with(Dialogue::from_field)]
    dialogue: Dialogue,
    #[sprite_sheet("char_red_1.png", 56, 56, 8, 1, 0, 0, 0)]
    sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: NPCColliderBundle,
}

fn name_from_field(entity_instance: &EntityInstance) -> Name {
    Name::new(
        entity_instance
            .get_string_field("name")
            .expect("expected entity to have non-nullable name string field")
            .clone(),
    )
}

// System to handle NPC dialogue triggers
pub fn handle_npc_dialogue_trigger(
    mut collision_events: EventReader<CollisionEvent>,
    npc_query: Query<&Dialogue>,
    player_query: Query<Entity, With<Player>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok(player) = player_query.get(*e1) {
                    if let Ok(dialogue) = npc_query.get(*e2) {
                        info!("NPC says: {}", dialogue.text);
                    }
                }
                // Check reverse entity order
                if let Ok(player) = player_query.get(*e2) {
                    if let Ok(dialogue) = npc_query.get(*e1) {
                        info!("NPC says: {}", dialogue.text);
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}

pub fn debug_npc_setup(
    npc_query: Query<(Entity, &Dialogue, &Name), With<NPC>>,
) {
    for (entity, dialogue, name) in npc_query.iter() {
        println!("NPC found: {:?} named {} with dialogue: {}", entity, name, dialogue.text);
    }
}

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, debug_npc_setup)
            .add_systems(Update, (
                spawn_npc_sensors,
                handle_npc_dialogue_trigger,
                // display_dialogue,
            ))
            .register_ldtk_entity::<NPCBundle>("NPC");
    }
}