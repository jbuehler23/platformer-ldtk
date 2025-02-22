use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{abilities::Abilities, dialogue::DialogueUI, items::{Item, ItemType}, npc::NPC, player::Player};

#[derive(Component)]
pub struct InteractionPrompt {
    pub entity: Entity, // The entity this prompt is associated with
    pub prompt_type: PromptType,
}

#[derive(PartialEq)]
pub enum PromptType {
    Dialogue,
    Pickup,
}

// System to spawn and position interaction prompts
pub fn handle_interaction_prompts(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    sensor_query: Query<(&Parent, &CollisionGroups)>,
    mut collision_events: EventReader<CollisionEvent>,
    item_query: Query<Entity, With<Item>>,
    npc_query: Query<Entity, With<NPC>>,
    prompt_query: Query<(Entity, &InteractionPrompt)>,
) {
    // Handle prompt spawning/positioning
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                if player_query.get(*e1).is_ok() || player_query.get(*e2).is_ok() {
                    // Check if collision is with an NPC or Item sensor
                    if let Ok((parent, _)) = sensor_query.get(*e1) {
                        let parent_entity = parent.get();

                        // Determine prompt type
                        let prompt_type = if npc_query.get(parent_entity).is_ok() {
                            Some(PromptType::Dialogue)
                        } else if item_query.get(parent_entity).is_ok() {
                            Some(PromptType::Pickup)
                        } else {
                            None
                        };

                        if let Some(prompt_type) = prompt_type {
                            // Spawn prompt UI
                            println!("Spawning prompt");
                            commands
                                .spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(30.0),
                                        ..Default::default()
                                    },
                                    BackgroundColor::from(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                                    InteractionPrompt {
                                        entity: parent_entity,
                                        prompt_type,
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Press E to interact"),
                                        TextFont {
                                            font_size: 16.0,
                                            ..Default::default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                // Remove prompts when out of range
                for (prompt_entity, _) in prompt_query.iter() {
                    commands.entity(prompt_entity).despawn_recursive();
                }
            }
        }
    }
}

// System to handle interaction input
pub fn handle_interaction_input(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    prompt_query: Query<(&InteractionPrompt, Entity)>,
    mut player_query: Query<&mut Abilities, With<Player>>,
    item_query: Query<(Entity, &Item)>,
    mut dialogue_ui_query: Query<&mut DialogueUI>,
    // mut inventory: ResMut<Inventory>, // You'll need to create this resource
) {
    if input.just_pressed(KeyCode::KeyE) {
        for (prompt, prompt_entity) in prompt_query.iter() {
            match prompt.prompt_type {
                PromptType::Dialogue => {
                    // Handle dialogue activation
                    if let Ok(mut dialogue_ui) = dialogue_ui_query.get_single_mut() {
                        dialogue_ui.active = true;
                    }
                }
                PromptType::Pickup => {
                    // Handle item pickup
                    // Add to inventory
                    // inventory.items.push(...);
                    
                    if let Ok((entity, item)) = item_query.get(prompt.entity) {
                        if let Ok(mut abilities) = player_query.get_single_mut() {
                            match item.item_type {
                                ItemType::SimpleShield => {
                                    abilities.unlock_block();
                                    println!("Unlocked block ability!");
                                }
                                // Handle other item types
                            }
                            // Despawn the item after pickup
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
            }
            // Remove the prompt
            commands.entity(prompt_entity).despawn_recursive();
        }
    }
}