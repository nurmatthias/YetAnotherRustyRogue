use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};
use serde::{Serialize, Deserialize};
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;


#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range : i32,
    pub dirty : bool
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Monster {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Name {
    pub name : String
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct CombatStats {
    pub max_hp : i32,
    pub hp : i32,
    pub defense : i32,
    pub power : i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToMelee {
    pub target : Entity
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct SufferDamage {
    pub amount : Vec<i32>
}
impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage { amount : vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}


// Items and handling
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Item {}
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Consumable {}
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Ranged {
    pub range: i32
}
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct ProvidesHealing {
    pub heal_amount: i32
}
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct ProvidesDamage {
    pub damage_amount: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct AreaOfEffect {
    pub radius: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Confusion {
    pub turns: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct InBackpack {
    pub owner: Entity
}

// things to do
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToPickup {
    pub collected_by: Entity,
    pub item: Entity
}
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToUse {
    pub item: Entity,
    pub target: Option<rltk::Point>
}
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToDrop {
    pub item: Entity
}

// invrastructure
pub struct SerializeThis {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map : super::map::Map
}