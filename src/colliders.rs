use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::{prelude::*, rapier::prelude::ColliderBuilder};

use crate::{items::Item, npc::NPC};

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
    pub active_events: ActiveEvents,
    pub collision_groups: CollisionGroups,
}

const PLAYER_GROUP: Group = Group::GROUP_1;
const NPC_GROUP: Group = Group::GROUP_2;
const WORLD_GROUP: Group = Group::GROUP_3;
const ENEMY_GROUP: Group = Group::GROUP_4;

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => {
                ColliderBundle {
                    collider: Collider::cuboid(12., 28.),
                    rigid_body: RigidBody::Dynamic,
                    velocity: Velocity::default(),
                    rotation_constraints,
                    gravity_scale: GravityScale(1.0),
                    friction: Friction::new(0.0),
                    density: ColliderMassProperties::Mass(1.0),
                    collision_groups: CollisionGroups::new(
                        PLAYER_GROUP,           // Collide with world
                        Group::ALL ^ NPC_GROUP, // Only affected by world
                    ),
                    active_events: ActiveEvents::COLLISION_EVENTS, // Enable collision detection
                }
            }
            // "NPC" => {
            //     ColliderBundle {
            //         collider: Collider::cuboid(12., 28.),
            //         rigid_body: RigidBody::Dynamic,
            //         velocity: Velocity::default(),
            //         rotation_constraints,
            //         gravity_scale: GravityScale(1.0),
            //         friction: Friction::new(0.0),
            //         density: ColliderMassProperties::Mass(1.0),
            //         collision_groups: CollisionGroups::new(
            //             NPC_GROUP | WORLD_GROUP, // Collide with world
            //             WORLD_GROUP              // Only affected by world
            //         ),
            //         active_events: ActiveEvents::COLLISION_EVENTS, // Enable collision detection
            //     }
            // }
            "Mob" => ColliderBundle {
                collider: Collider::cuboid(5., 5.),
                rigid_body: RigidBody::KinematicVelocityBased,
                rotation_constraints,
                ..Default::default()
            },
            "Chest" => ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale: GravityScale(1.0),
                friction: Friction::new(0.5),
                density: ColliderMassProperties::Density(15.0),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    // pub collider: Collider,
    // pub sensor: Sensor,
    // pub active_events: ActiveEvents,
    // pub rotation_constraints: LockedAxes,
}

impl From<IntGridCell> for SensorBundle {
    fn from(int_grid_cell: IntGridCell) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        // ladder
        if int_grid_cell.value == 2 {
            SensorBundle {
                // collider: Collider::cuboid(8., 8.),
                // sensor: Sensor,
                // rotation_constraints,
                // active_events: ActiveEvents::COLLISION_EVENTS,
            }
        } else {
            SensorBundle::default()
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct NPCColliderBundle {
    pub collider: Collider,
    // pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
    pub collision_groups: CollisionGroups,
    pub rigid_body: RigidBody,
    pub gravity_sale: GravityScale,
}

impl From<&EntityInstance> for NPCColliderBundle {
    fn from(entity_instance: &EntityInstance) -> NPCColliderBundle {
        match entity_instance.identifier.as_ref() {
            "NPC" => NPCColliderBundle {
                collider: Collider::cuboid(12., 28.),
                // sensor: Sensor,
                rigid_body: RigidBody::Dynamic,
                gravity_sale: GravityScale(1.0),
                active_events: ActiveEvents::COLLISION_EVENTS,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                collision_groups: CollisionGroups::new(
                    WORLD_GROUP,
                    WORLD_GROUP,
                ),
            },
            _ => NPCColliderBundle::default(),
        }
    }
}

// Add this system to spawn sensor children
pub fn spawn_npc_sensors(mut commands: Commands, query: Query<Entity, Added<NPC>>) {
    for npc_entity in query.iter() {
        commands.entity(npc_entity).with_children(|parent| {
            parent.spawn((
                Collider::cuboid(20., 30.), // Slightly larger trigger area
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(
                    PLAYER_GROUP, // NPC interaction group
                    PLAYER_GROUP, // Only detect player collisions
                ),
            ));
        });
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ItemColliderBundle {
    pub collider: Collider,
    // pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
    pub collision_groups: CollisionGroups,
    pub rigid_body: RigidBody,
    pub gravity_sale: GravityScale,
}

impl From<&EntityInstance> for ItemColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ItemColliderBundle {
        match entity_instance.identifier.as_ref() {
            "Item" => ItemColliderBundle {
                collider: Collider::cuboid(8., 8.),
                // sensor: Sensor,
                rigid_body: RigidBody::Dynamic,
                gravity_sale: GravityScale(1.0),
                active_events: ActiveEvents::COLLISION_EVENTS,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                collision_groups: CollisionGroups::new(
                    WORLD_GROUP, // NPC collision group
                    WORLD_GROUP, // Only detect player collisions
                ),
            },
            _ => ItemColliderBundle::default(),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct GolemColliderBundle {
    pub collider: Collider,
    // pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
    pub collision_groups: CollisionGroups,
    pub rigid_body: RigidBody,
    pub gravity_sale: GravityScale,
    pub velocity: Velocity,
}

impl From<&EntityInstance> for GolemColliderBundle {
    fn from(entity_instance: &EntityInstance) -> GolemColliderBundle {
        match entity_instance.identifier.as_ref() {
            "Golem" => GolemColliderBundle {
                collider: Collider::cuboid(26., 26.),
                // sensor: Sensor,
                rigid_body: RigidBody::KinematicVelocityBased,
                gravity_sale: GravityScale(1.0),
                active_events: ActiveEvents::COLLISION_EVENTS,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                collision_groups: CollisionGroups::new(
                    PLAYER_GROUP, // NPC collision group
                    PLAYER_GROUP, // Only detect player collisions
                ),
                velocity: Velocity::default(),
            },
            _ => GolemColliderBundle::default(),
        }
    }
}

// Add this system to spawn sensor children
pub fn spawn_item_sensors(mut commands: Commands, query: Query<Entity, Added<Item>>) {
    for npc_entity in query.iter() {
        commands.entity(npc_entity).with_children(|parent| {
            parent.spawn((
                Collider::cuboid(8., 8.), // Slightly larger trigger area
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(
                    PLAYER_GROUP, // NPC interaction group
                    PLAYER_GROUP, // Only detect player collisions
                ),
            ));
        });
    }
}
