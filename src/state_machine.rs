use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{animation, ground_detection::GroundDetection, player::Player};

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Running,
    Attacking,
    Jumping,
    Falling,
    Blocking
}

#[derive(Event, Debug, Clone)]
pub enum PlayerEvent {
    MovementStarted(MovementType),
    AttackStarted(AttackType),
    AnimationCompleted(AnimationType),
    BlockStarted,
    BlockEnded,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MovementType {
    Idle,
    Run(Direction), // Direction
    Jump,
    Fall,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttackType {
    Melee,
    Ranged,
    Special,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationType {
    Idle,
    Running,
    Jumping,
    Falling,
    Attacking,
    Blocking
}

pub fn player_state_transition(
    mut next_state: ResMut<NextState<PlayerState>>,
    mut player_events: EventReader<PlayerEvent>,
    state: Res<State<PlayerState>>,
) {
    for event in player_events.read() {
        match event {
            PlayerEvent::MovementStarted(movement) => {
                // Only process movement events if not attacking
                if *state.get() != PlayerState::Attacking {
                    match movement {
                        MovementType::Idle => next_state.set(PlayerState::Idle),
                        MovementType::Run(_) => next_state.set(PlayerState::Running),
                        MovementType::Jump => next_state.set(PlayerState::Jumping),
                        MovementType::Fall => next_state.set(PlayerState::Falling),
                    }
                }
            },
            PlayerEvent::AttackStarted(_) => {
                next_state.set(PlayerState::Attacking);
            },
            PlayerEvent::AnimationCompleted(AnimationType::Attacking) => {
                next_state.set(PlayerState::Idle);
            },
            PlayerEvent::BlockStarted => {
                next_state.set(PlayerState::Blocking);
            }
            PlayerEvent::BlockEnded => {
                next_state.set(PlayerState::Idle);
            }
            _ => (),
        }
    }
}