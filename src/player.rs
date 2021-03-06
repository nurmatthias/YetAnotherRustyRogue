use rltk::{VirtualKeyCode, Rltk, Point};
use specs::prelude::*;
use std::cmp::{max, min};
use super::{Position, Player, Viewshed, State, Map, RunState, CombatStats, Item, WantsToMelee, WantsToPickup, TileType, Monster};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) -> Option<RunState> {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let items = ecs.read_storage::<Item>();
    let map = ecs.fetch::<Map>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let mut wants_to_pickup = ecs.write_storage::<WantsToPickup>();

    for (entity, _player, pos, viewshed) in (&entities, &players, &mut positions, &mut viewsheds).join() {
        if pos.x + delta_x < 1 || pos.x + delta_x > map.width-1 || pos.y + delta_y < 1 || pos.y + delta_y > map.height-1 { 
            break; 
        }
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target }).expect("Add target failed");
                break;
            }

            let item = items.get(*potential_target);
            if let Some(_item) = item {
                wants_to_pickup.insert(entity, WantsToPickup { collected_by: entity, item: *potential_target }).expect("Pickup failed");
                break;
            }
        }

        if !map.blocked_tiles[destination_idx] {
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }

        if try_next_level(ecs) {
            return Some(RunState::NextLevel);
        }
    }

    None
}

pub fn try_next_level(ecs: &World) -> bool {

    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);

    map.tiles[player_idx] == TileType::DownStairs
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    let next_runstate: Option<RunState>;
    match ctx.key {
        None => { return RunState::AwaitingInput } // Nothing happened
        Some(key) => match key {

            // Cardinal movement
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => next_runstate = try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => next_runstate = try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K => next_runstate = try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => next_runstate = try_move_player(0, 1, &mut gs.ecs),

            // Diagonal movement
            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::U => next_runstate = try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 |
            VirtualKeyCode::Y => next_runstate = try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::N => next_runstate = try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 |
            VirtualKeyCode::B => next_runstate = try_move_player(-1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad5 |
            VirtualKeyCode::Space => next_runstate = skip_turn(&mut gs.ecs),

            // Menu and stuff
            VirtualKeyCode::I => next_runstate = Some(RunState::ShowInventory),
            VirtualKeyCode::D => next_runstate = Some(RunState::ShowDropInventory),
            VirtualKeyCode::Escape => next_runstate = Some(RunState::SaveGame),

            _ => { next_runstate = Some(RunState::AwaitingInput) }
        },
    }

    next_runstate.unwrap_or(RunState::PlayerTurn)
}


fn skip_turn(ecs: &mut World) -> Option<RunState> {

    let player_entity = ecs.fetch::<Entity>();
    let viewshed_comp = ecs.read_storage::<Viewshed>();
    let monsters = ecs.read_storage::<Monster>();

    let map = ecs.fetch::<Map>();

    let mut can_heal = true;
    let viewshed = viewshed_comp.get(*player_entity).unwrap();
    for tile in viewshed.visible_tiles.iter() {
        let idx = map.xy_idx(tile.x, tile.y);
        for entity_id in map.tile_content[idx].iter() {
            let mob = monsters.get(*entity_id);
            match mob {
                None => {}
                Some(_) => { can_heal = false; }
            }
        }
    }

    if can_heal {
        let mut health_comp = ecs.write_storage::<CombatStats>();
        let player_hp = health_comp.get_mut(*player_entity).unwrap();
        player_hp.hp = i32::min(player_hp.hp + 1, player_hp.max_hp);
    }

    Some(RunState::PlayerTurn)
}
