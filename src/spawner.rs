use rltk::{RGB, RandomNumberGenerator};
use specs::prelude::*;

use super::*;


const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;


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
        .build()
}

pub fn spawn_room(ecs: &mut World, room : &Rect) {
    let mut monster_spawn_points : Vec<usize> = Vec::new();
    let mut item_spawn_points : Vec<usize> = Vec::new();

    // Scope to keep the borrow checker happy
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS + 2) -3;

        for _i in 0 .. num_monsters {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        for _i in 0..num_items {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !item_spawn_points.contains(&idx) {
                    item_spawn_points.push(idx);
                    added = true;
                }
            }
        }
    }

    for idx in monster_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(ecs, x as i32, y as i32);
    }

    for idx in item_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_item(ecs, x as i32, y as i32);
    }
}


fn random_monster(ecs: &mut World, pos_x: i32, pos_y: i32) {

    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }

    match roll {
        1 => { orc(ecs, pos_x, pos_y) }
        _ => { goblin(ecs, pos_x, pos_y) }
    }
}


fn random_item(ecs: &mut World, pos_x: i32, pos_y: i32) {

    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 5);
    }

    match roll {
        1 => { magic_missile_scroll(ecs, pos_x, pos_y); }
        2 => { fireball_scroll(ecs, pos_x, pos_y); }
        3 => { confusion_scroll(ecs, pos_x, pos_y); }
        _ => { health_potion(ecs, pos_x, pos_y); }
    }
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
        .build();
}