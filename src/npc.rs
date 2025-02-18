use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{colliders::{spawn_npc_sensors, ColliderBundle, NPCColliderBundle}, dialogue::{spawn_dialogue_ui, update_dialogue_ui, Dialogue, DialogueUI}, player::{self, Player}};

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
    transform: Transform,
    global_transform: GlobalTransform,
}

fn name_from_field(entity_instance: &EntityInstance) -> Name {
    Name::new(
        entity_instance
            .get_string_field("name")
            .expect("expected entity to have non-nullable name string field")
            .clone(),
    )
}

// Update the NPC dialogue trigger system
pub fn handle_npc_dialogue_trigger(
    mut collision_events: EventReader<CollisionEvent>,
    npc_query: Query<(&Dialogue, &Name), With<NPC>>,
    player_query: Query<Entity, With<Player>>,
    sensor_query: Query<&Parent, With<Sensor>>,
    mut dialogue_ui_query: Query<&mut DialogueUI>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let sensor_parent = get_sensor_parent(*e1, *e2, &sensor_query);
                let is_player = is_player_involved(*e1, *e2, &player_query);

                if let (Some(npc_entity), true) = (sensor_parent, is_player) {
                    if let Ok((dialogue, name)) = npc_query.get(npc_entity) {
                        if let Ok(mut dialogue_ui) = dialogue_ui_query.get_single_mut() {
                            dialogue_ui.active = true;
                            dialogue_ui.text = format!("{}: {}", name, dialogue.text);
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                if let Ok(mut dialogue_ui) = dialogue_ui_query.get_single_mut() {
                    dialogue_ui.active = false;
                }
            }
        }
    }
}

/// Helper function to check if either entity is the player
fn is_player_involved(entity1: Entity, entity2: Entity, player_query: &Query<Entity, With<Player>>) -> bool {
    player_query.get(entity1).is_ok() || player_query.get(entity2).is_ok()
}

/// Helper function to get the parent entity of a sensor if it exists
fn get_sensor_parent(entity1: Entity, entity2: Entity, sensor_query: &Query<&Parent, With<Sensor>>) -> Option<Entity> {
    sensor_query.get(entity1)
        .map(|parent| parent.get())
        .ok()
        .or_else(|| sensor_query.get(entity2).map(|parent| parent.get()).ok())
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
            .add_systems(Startup, spawn_dialogue_ui)
            .add_systems(Update, (
                // debug_npc_setup,
                spawn_npc_sensors,
                handle_npc_dialogue_trigger,
                update_dialogue_ui,
            ))
            .register_ldtk_entity::<NPCBundle>("NPC");
    }
}