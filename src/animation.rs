use bevy::{prelude::*, utils::HashMap};

use crate::{player::Player, state_machine::{AnimationType, PlayerEvent, PlayerState}};

#[derive(Component, Clone, Debug)]
pub struct PlayerAnimation {
    pub timer: Timer,
    frames: HashMap<PlayerState, (usize, usize)>, // (start_frame, num_frames, should_loop)
    current_frame_count: usize,
}


impl Default for PlayerAnimation {
    fn default() -> Self {
        let mut frames = HashMap::new();
        frames.insert(PlayerState::Idle, (0, 6));      // Loops
        frames.insert(PlayerState::Running, (16, 8));  // Loops
        frames.insert(PlayerState::Attacking, (8, 6)); // Plays once
        frames.insert(PlayerState::Jumping, (24, 9));  // Loops
        frames.insert(PlayerState::Falling, (33, 7));  // Loops
        frames.insert(PlayerState::Blocking, (80, 3));  // Add blocking animation frames
        
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frames,
            current_frame_count: 0,
        }
    }
}

pub fn animate_player_sprite(
    mut query: Query<(&mut Sprite, &mut PlayerAnimation), With<Player>>,
    state: Res<State<PlayerState>>,
    time: Res<Time>,
    mut player_events: EventWriter<PlayerEvent>,
) {
    for (mut sprite, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if let Some(texture_atlas) = &mut sprite.texture_atlas {
                let (start_frame, num_frames) = *animation
                    .frames
                    .get(state.get())
                    .unwrap_or(&(0, 6));

                if texture_atlas.index < start_frame || 
                   texture_atlas.index >= start_frame + num_frames {
                    animation.current_frame_count = 0;
                }

                // Non-looping animations send completion events
                match state.get() {
                    PlayerState::Attacking if animation.current_frame_count >= num_frames => {
                        player_events.send(PlayerEvent::AnimationCompleted(
                            AnimationType::Attacking
                        ));
                    },
                    PlayerState::Blocking if animation.current_frame_count >= num_frames => {
                        player_events.send(PlayerEvent::AnimationCompleted(
                            AnimationType::Blocking
                        ));
                    },
                    _ => {}
                }

                let next_frame = match state.get() {
                    // Non-looping animations
                    PlayerState::Attacking | PlayerState::Blocking => {
                        start_frame + animation.current_frame_count.min(num_frames - 1)
                    },
                    // Looping animations
                    _ => start_frame + (animation.current_frame_count % num_frames),
                };

                texture_atlas.index = next_frame;
                animation.current_frame_count += 1;
            }
        }
    }
}
