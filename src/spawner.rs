use rltk::{ RGB, RandomNumberGenerator };
use specs::prelude::*;
use super::{CombatStats, Player, Renderable, Name, Position, Viewshed, Monster, BlocksTile, Rect, Item,
    Consumable, Ranged, ProvidesHealing, map::MAPWIDTH, ProvidesDamage, AreaOfEffect, Confusion, SerializeThis,
    spawn_table::RandomTable, InBackpack};
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::collections::HashMap;


const MAX_ENTITIES: i32 = 4;


// Spawn a player entity
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {

    ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Name{name: "Player".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub fn spawn_room(ecs: &mut World, room : &Rect, map_depth: i32) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();

    // Scope to keep the borrow checker happy
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_spawns = rng.roll_dice(1, MAX_ENTITIES + 3) + (map_depth - 1) - 3;

        for _i in 0 .. num_spawns {
            let mut added = false;
            let mut tries = 0;
            while !added && tries < 20 {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !spawn_points.contains_key(&idx) {
                    spawn_points.insert(idx, spawn_table.roll(&mut rng));
                    added = true;
                } else {
                    tries += 1;
                }
            }
        }
    }

    for spawn in spawn_points.iter() {
        let x = (*spawn.0 % MAPWIDTH) as i32;
        let y = (*spawn.0 / MAPWIDTH) as i32;

        match spawn.1.as_ref() {
            "Goblin" => goblin(ecs, x, y),
            "Orc" => orc(ecs, x, y),
            "Health Potion" => _ = health_potion(ecs, x, y),
            "Fireball Scroll" => _ = fireball_scroll(ecs, x, y),
            "Confusion Scroll" => _ = confusion_scroll(ecs, x, y),
            "Magic Missile Scroll" => _ = magic_missile_scroll(ecs, x, y),
            _ => {}
        }
    }
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1 + map_depth)
        .add("Health Potion", 7)
        .add("Fireball SWcroll", 2 + map_depth)
        .add("Confusion Scroll", 2 + map_depth)
        .add("Magic Missile Scroll", 4)
}


pub fn create_test_backpack(ecs: &mut World) {

    let mut items = Vec::new();
    items.push(health_potion(ecs, 0, 0));
    items.push(health_potion(ecs, 0, 0));
    items.push(magic_missile_scroll(ecs, 0, 0));
    items.push(magic_missile_scroll(ecs, 0, 0));
    items.push(fireball_scroll(ecs, 0, 0));
    items.push(fireball_scroll(ecs, 0, 0));
    items.push(confusion_scroll(ecs, 0, 0));
    items.push(confusion_scroll(ecs, 0, 0));

    let player = ecs.read_resource::<Entity>();
    let mut positions = ecs.write_storage::<Position>();
    let mut backpack = ecs.write_storage::<InBackpack>();

    for item in items.iter() {
        positions.remove(*item);
        backpack.insert(*item, InBackpack { owner: *player }).expect("Unable to insert Item in backpack");
    }
}

fn health_potion(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {x, y})
        .with(Renderable {
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name { name: "Health Potion".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesHealing { heal_amount: 8 })
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {x, y})
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name { name: "Magic Missle Scroll".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesDamage { damage_amount: 8 })
        .with(Ranged{ range: 6})
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

fn fireball_scroll(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {x, y})
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name { name: "Fireball Scroll".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesDamage { damage_amount: 20 })
        .with(Ranged{ range: 6})
        .with(AreaOfEffect{ radius: 3 })
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

fn confusion_scroll(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {x, y})
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::PINK),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name { name: "Confusion Scroll".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6})
        .with(Confusion{ turns: 4 })
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}


fn orc(ecs: &mut World, pos_x: i32, pos_y: i32) {
    monster(ecs, pos_x, pos_y, rltk::to_cp437('o'), "Ork");
}

fn goblin(ecs: &mut World, pos_x: i32, pos_y: i32) {
    monster(ecs, pos_x, pos_y, rltk::to_cp437('g'), "Goblin");
}

fn monster<S : ToString>(ecs: &mut World, pos_x: i32, pos_y: i32, glyph: rltk::FontCharType, name: S) {
    ecs.create_entity()
        .with(Position{ x: pos_x, y: pos_y })
        .with(Renderable{
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Monster{})
        .with(Name{ name: name.to_string() })
        .with(BlocksTile{})
        .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
        .marked::<SimpleMarker<SerializeThis>>()
        .build();
}