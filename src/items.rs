use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::colliders::{spawn_item_sensors, ItemColliderBundle};

#[derive(Debug, Clone, Default, Component, Reflect)]
pub struct Item {
    pub item_type: ItemType,
    pub count: u32,
}


#[derive(Debug, Clone, Default, Component, Reflect)]
pub enum ItemType {
    #[default]
    SimpleShield,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ItemBundle {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ItemColliderBundle,
    #[with(Item::from_field)]
    pub item: Item,
}

impl FromStr for ItemType {

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s {
            "Simple_Shield" => Ok(ItemType::SimpleShield),
            _ => Ok(ItemType::SimpleShield),
        }
    }
    
    type Err = String;
}

impl Item {
    pub fn from_field(entity_instance: &EntityInstance) -> Item {
        let item_type_field = entity_instance.get_enum_field("type")
            .expect("expected entity to have non-nullable type enums field");
        let item_type = ItemType::from_str(&item_type_field)
            .unwrap_or(ItemType::SimpleShield);
        let count_field = entity_instance.get_int_field("count")
            .expect("expected entity to have non-nullable count int field");
        let count = *count_field as u32;

        println!("Found item: {:?}", item_type);

        Item {
            item_type,
            count,
        }
        
    }
}

#[derive(Debug, Default, Component, Reflect, Clone)]
pub struct Loot {
    drops: Vec<ItemType>,
}

impl Loot {
    pub fn from_field(entity_instance: &EntityInstance) -> Loot {
        let drops = entity_instance
            .iter_enums_field("loot")
            .expect("expected entity to have non-nullable loot enums field")
            .map(|field| ItemType::from_str(field))
            .collect::<Result<_, _>>()
            .unwrap();

            Loot { drops }
    }
}

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Item>()
            .add_systems(Update, spawn_item_sensors)
            .register_ldtk_entity::<ItemBundle>("Item");
    }
}