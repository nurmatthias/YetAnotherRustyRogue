use specs::prelude::*;
use crate::Name;

use super::{CombatStats, SufferDamage, Player, gamelog::GameLog};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = ( 
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

pub fn delete_the_dead(ecs : &mut World) {
    let mut dead : Vec<Entity> = Vec::new();
    // scoping some functions
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let names = ecs.read_storage::<Name>();
        let entities = ecs.entities();
        let mut log = ecs.write_resource::<GameLog>();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                let player = players.get(entity);
                let name = names.get(entity);
                match player {
                    None => {
                        if let Some(name) = name {log.entries.push(format!("{} died in pain!", name.name))}
                        dead.push(entity);
                    }
                    Some(_) => log.entries.push("You are dead".to_string())
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete entity");
    }
}
