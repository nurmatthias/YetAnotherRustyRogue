use specs::prelude::*;

use super::{WantsToPickup, Name, InBackpack, Position, gamelog::GameLog, Potion, WantsToUse, CombatStats, WantsToDrop};


pub struct ItemCollectionSystem{}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToPickup>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, mut wants_pickup, mut positions, names, mut backpacks) = data;

        for pickup in wants_pickup.join() {

            positions.remove(pickup.item);
            backpacks.insert(pickup.item, InBackpack { owner: pickup.collected_by }).expect("Unable to insert Item in backpack");

            if pickup.collected_by == *player_entity {
                gamelog.entries.push(format!("You picked up the {}", names.get(pickup.item).unwrap().name));
            }

        }

        wants_pickup.clear();
    }
}


pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {

    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToUse>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Potion>,
        WriteStorage<'a, CombatStats>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (player_entity, mut gamelog, entities, mut wants_use, names, potions, mut combat_stats) = data;

        for (entity, item_use, stats) in (&entities, &wants_use, &mut combat_stats).join() {
            let potion = potions.get(item_use.item);
            match potion {
                None => {}
                Some(potion) => {
                    stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);
                    if entity == *player_entity {
                        gamelog.entries.push(format!("You drink the {}, healing {} hp.", names.get(item_use.item).unwrap().name, potion.heal_amount));
                    }
                    entities.delete(item_use.item).expect("Delete failed");
                }
            }
        }

        wants_use.clear();
    }
}


pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {

    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToDrop>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, InBackpack>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, entities, mut want_to_drop, names, mut positions, mut backpack) = data;

        for (entity, to_drop) in (&entities, &want_to_drop).join() {
            let mut drop_pos: Position = Position{x:0, y:0};
            {
                let drop_position = positions.get(entity).unwrap();
                drop_pos.x = drop_position.x;
                drop_pos.y = drop_position.y;
            }

            positions.insert(to_drop.item, Position{ x: drop_pos.x, y: drop_pos.y}).expect("Unable to position item there");
            backpack.remove(to_drop.item);

            if entity == *player_entity {
                gamelog.entries.push(format!("You drop the {}", names.get(to_drop.item).unwrap().name));
            }
        }

        want_to_drop.clear();
    }
}