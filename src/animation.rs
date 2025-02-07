use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::PlayerBundle;

#[derive(Component, Clone, Debug)]
pub struct IdleAnimation {
    current_frame: usize,
    num_frames: usize,
    timer: Timer,
    base_index: Option<usize>,
}

impl Default for IdleAnimation {
    fn default() -> Self {
        Self {
            base_index: None,
            current_frame: 0,
            num_frames: 6,
            timer: Timer::from_seconds(
                0.1,
                TimerMode::Repeating,
            ),
        }
    }
}

pub fn idle_animation(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut IdleAnimation)>,
) {
    for (mut sprite, mut idle) in query.iter_mut() {
        idle.timer.tick(time.delta());
        
        if idle.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % idle.num_frames;
            }
        }
    }
}