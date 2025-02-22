// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

mod camera;
mod climbing;
// /// Bundles for auto-loading Rapier colliders as part of the level
mod colliders;
mod npc;
// mod enemy;
// /// Handles initialization and switching levels
mod game_flow;
mod ground_detection;
// mod inventory;
// mod misc_objects;
mod animation;
mod player;
mod walls;
mod state_machine;
mod dialogue;
mod items;
mod boss;
mod health;
mod interaction;
mod abilities;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .add_plugins(RapierDebugRenderPlugin {
            // Customize the debug render settings
            mode: DebugRenderMode::COLLIDER_SHAPES | DebugRenderMode::RIGID_BODY_AXES,
            style: DebugRenderStyle {
                rigid_body_axes_length: 20.0,
                ..default()
            },
            ..default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(game_flow::GameFlowPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(climbing::ClimbingPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(npc::NPCPlugin)
        .add_plugins(items::ItemsPlugin)
        .add_plugins(boss::BossesPlugin)
        // .add_plugins(enemy::EnemyPlugin)
        // .add_systems(Update, inventory::dbg_print_inventory)
        .add_systems(Update, (
            camera::camera_fit_inside_current_level,
            (
                toggle_debug,
                (|mut debug_render_context: ResMut<DebugRenderContext>| {
                    debug_render_context.enabled = !debug_render_context.enabled;
                })
                .run_if(input_just_pressed(KeyCode::KeyV)),
            )
        ))
        // .add_plugins(misc_objects::MiscObjectsPlugin)
        .run();
}

#[derive(Component)]
pub struct DebugCooldown(pub Timer);

pub fn toggle_debug(time: Res<Time>, mut query: Query<(&mut ColliderDebug, &mut DebugCooldown)>) {
    for (mut debug, mut cooldown) in query.iter_mut() {
        cooldown.0.tick(time.delta());
        if cooldown.0.just_finished() {
            *debug = match *debug {
                ColliderDebug::AlwaysRender => ColliderDebug::NeverRender,
                ColliderDebug::NeverRender => ColliderDebug::AlwaysRender,
            }
        }
    }
}

