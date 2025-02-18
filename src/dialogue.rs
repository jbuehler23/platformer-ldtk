use bevy::{prelude::*, text::FontStyle};
use bevy_ecs_ldtk::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct Dialogue {
    pub text: String,
}

#[derive(Component)]
pub struct DialogueUI {
    pub active: bool,
    pub text: String,
}

// System to spawn dialogue UI
pub fn spawn_dialogue_ui(mut commands: Commands) {
    // Spawn hidden dialogue box
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Px(20.0),
                right: Val::Px(20.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            Visibility::Hidden,
            BackgroundColor::from(Color::srgba(0.2, 0.2, 0.2, 0.8)),
            DialogueUI {
                active: false,
                text: String::new(),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

// System to update dialogue UI
pub fn update_dialogue_ui(
    mut dialogue_query: Query<(&mut Visibility, &DialogueUI, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (mut visibility, dialogue, children) in dialogue_query.iter_mut() {
        if dialogue.active {
            *visibility = Visibility::Visible;
            
            // Update text
            if let Ok(mut text) = text_query.get_mut(children[0]) {
                text.0 = dialogue.text.clone();
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
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