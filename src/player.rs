use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::animation::{animate_player_sprite, PlayerAnimation};
// use crate::{climbing::Climber, inventory::Inventory};
use crate::climbing::Climber;
use crate::state_machine::{player_state_transition, AttackType, Direction, MovementType, PlayerEvent, PlayerState};
use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component,)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet("char_green_1.png", 56, 56, 8, 8, 0, 0, 0)]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub climber: Climber,
    pub ground_detection: GroundDetection,
    // Add Transform and GlobalTransform
    // pub transform: Transform,

    // Build Items Component manually by using `impl From<&EntityInstance>`
    // #[from_entity_instance]
    // items: Inventory,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
    animation: PlayerAnimation,

}

pub fn handle_player_movement_and_input(
    mut query: Query<(&mut Velocity, &GroundDetection, &mut Climber), With<Player>>,
    mut player_events: EventWriter<PlayerEvent>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut velocity, ground_detection, mut climber)) = query.get_single_mut() else { return };
    
    // Handle horizontal movement
    let mut direction = 0.0;
    if input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    // Apply horizontal movement
    if direction != 0.0 {
        velocity.linvel.x = direction * 200.;
        player_events.send(PlayerEvent::MovementStarted(
            MovementType::Run(if direction > 0.0 { Direction::Right } else { Direction::Left })
        ));
    } else if ground_detection.on_ground {
        velocity.linvel.x = 0.;
        player_events.send(PlayerEvent::MovementStarted(MovementType::Idle));
    }

    // Handle jumping
    if (input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::ArrowUp)) 
        && ground_detection.on_ground 
        && !climber.climbing 
    {
        velocity.linvel.y = 500.;
        player_events.send(PlayerEvent::MovementStarted(MovementType::Jump));
        climber.climbing = false;
    }

    // Handle falling
    if !ground_detection.on_ground && velocity.linvel.y < 0.0 {
        player_events.send(PlayerEvent::MovementStarted(MovementType::Fall));
    }

    // Handle attack input
    if input.just_pressed(KeyCode::KeyA) {
        player_events.send(PlayerEvent::AttackStarted(AttackType::Melee));
    }
}

pub fn handle_sprite_direction(
    mut query: Query<&mut Sprite, With<Player>>,
    mut player_events: EventReader<PlayerEvent>,
) {
    let Ok(mut sprite) = query.get_single_mut() else { return };
    
    for event in player_events.read() {
        if let PlayerEvent::MovementStarted(MovementType::Run(direction)) = event {
            sprite.flip_x = matches!(direction, Direction::Left);
        }
    }
}



fn handle_player_children(
    mut commands: Commands,
    new_players: Query<Entity, Added<Player>>,
)
{
    // for player_entity in new_players.iter() {
    //     let sprite_height = 56.0;
    //     let sprite_width = 56.0;
    //     let collider_height = 40.0;
    //     let collider_width = 30.0;
        
    //     let sprite_y_offset = (collider_height - sprite_height) / 2.0;
    //     let vertical_offset = (sprite_height - collider_height) / 2.0 + 8.0;
    //     let horizontal_offset = (sprite_width - collider_width) / 2.0;

    //     // Calculate offsets
    //     let sprite_y_offset = (collider_height - sprite_height) / 2.0;
        
    //     commands.entity(player_entity)
    //         // Don't modify the parent transform - it will mess with the sprite
    //         .with_children(|parent| {
    //             parent.spawn((
    //                 ColliderBundle {
    //                     collider: Collider::cuboid(collider_width / 2.0, collider_height / 2.0),
    //                     rigid_body: RigidBody::Dynamic,
    //                     velocity: Velocity::default(),
    //                     rotation_constraints: LockedAxes::ROTATION_LOCKED,
    //                     gravity_scale: GravityScale(1.0),
    //                     friction: Friction::new(0.0),
    //                     density: ColliderMassProperties::Mass(1.0),
    //                 },
    //                 // Apply the offset to the collider instead
    //                 Transform::from_xyz(0.0, 8.0, 0.0),
    //                 GlobalTransform::default(),
    //             ));
    //         });
    // }

}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<PlayerState>()
            .add_event::<PlayerEvent>()
            .add_systems(Update, (
                handle_player_children,
               handle_player_movement_and_input,
               player_state_transition,
               handle_sprite_direction,
               animate_player_sprite
            ).chain())
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}
