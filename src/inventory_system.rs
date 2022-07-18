use specs::prelude::*;

use super::*;


pub struct ItemCollectionSystem{}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, gamelog::GameLog>,
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
        WriteExpect<'a, gamelog::GameLog>,
        ReadExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, WantsToUse>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Consumable>,
        ReadStorage<'a, ProvidesHealing>,
        ReadStorage<'a, ProvidesDamage>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, AreaOfEffect>,
        WriteStorage<'a, Confusion>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (player_entity, mut gamelog, map, entities, mut wants_to_use, names, consumables, healing, damage, mut combat_stats, mut suffer_damage, aoe, mut confusion) = data;

        for (entity, use_item) in (&entities, &wants_to_use).join() {
            let mut item_used = false;

            // target
            let mut targets: Vec<Entity> = Vec::new();
            match use_item.target {
                None => { targets.push( *player_entity ); }
                Some(target) => {
                    let area_of_effect = aoe.get(use_item.item);
                    match area_of_effect {
                        None => {
                            let idx = map.xy_idx(target.x, target.y);
                            for mob in map.tile_content[idx].iter() {
                                targets.push(*mob);
                            }
                        }
                        Some(area_of_effect) => {
                            let mut effected_tiles = rltk::field_of_view(target, area_of_effect.radius, &*map);
                            effected_tiles.retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height -1);
                            for tile_idx in effected_tiles.iter() {
                                let idx = map.xy_idx(tile_idx.x, tile_idx.y);
                                for mob in map.tile_content[idx].iter() {
                                    targets.push(*mob);
                                }
                            }
                        }
                    }
                }
            }


            let item_heals = healing.get(use_item.item);
            match item_heals {
                None => {}
                Some(heals) => {
                    for target in targets.iter() {
                        let stats = combat_stats.get_mut(*target);
                        if let Some(stats) = stats {
                            stats.hp = i32::min(stats.max_hp, stats.hp + heals.heal_amount);
                            if entity == *player_entity {
                                gamelog.entries.push(format!("You drink the {}, healing {} hp.", names.get(use_item.item).unwrap().name, heals.heal_amount));
                            }
                        }
                    }

                    item_used = true;
                }
            }

            let item_damage = damage.get(use_item.item);
            match item_damage {
                None => {}
                Some(damage) => {
                    for mob in targets.iter() {
                        SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage_amount);
                        if entity == *player_entity {
                            let mob_name = names.get(*mob).unwrap();
                            let item_name = names.get(use_item.item).unwrap();
                            gamelog.entries.push(format!("You use {} on {}, inflicting {} damage.", item_name.name, mob_name.name, damage.damage_amount))
                        }
                    }

                    item_used = true;
                }
            }

            let mut add_confusion = Vec::new();
            {
                let item_confusion = confusion.get(use_item.item);
                match item_confusion {
                    None => {}
                    Some(confusion) => {
                        for mob in targets.iter() {
                            add_confusion.push((*mob, confusion.turns));
                            if entity == *player_entity {
                                let mob_name = names.get(*mob).unwrap();
                                let item_name = names.get(use_item.item).unwrap();
                                gamelog.entries.push(format!("You use {} on {}, confusiong them.", item_name.name, mob_name.name))
                            }
                        }

                        item_used = true;
                    }
                }
            }
            for mob in add_confusion.iter() {
                confusion.insert(mob.0, Confusion{turns: mob.1}).expect("Unable to add confusion");
            }

            let consumable = consumables.get(use_item.item);
            match consumable {
                None => {}
                Some(_) => {
                    if item_used {
                        entities.delete(use_item.item).expect("Delete failed");
                    }
                }
            }
        }

        wants_to_use.clear();
    }
}


pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {

    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, gamelog::GameLog>,
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