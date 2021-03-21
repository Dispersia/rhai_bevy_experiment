mod script_manager;

use std::fmt::Display;

use script_manager::{BehaviorType, ScriptManager};

use bevy_ecs::prelude::*;
use rhai::{Array, Dynamic, EvalAltResult, Scope};

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut world = World::new();
    world.spawn_batch(
        (0..2)
            .map(|i| {
                (
                    Health { value: i * 10 },
                    Position {
                        x: i as f32 * 10.,
                        y: i as f32 * 20.,
                    },
                )
            })
            .collect::<Vec<_>>(),
    );

    let mut scope = Scope::new();

    scope.push("world", std::sync::Arc::new(world));

    let script_manager = create_script_manager()?;

    let result = script_manager.get_script(1, BehaviorType::Combat).unwrap();
    script_manager
        .get_engine()
        .eval_ast_with_scope(&mut scope, &result)?;

    Ok(())
}

#[derive(Default, Bundle, Clone, Debug)]
pub struct Health {
    value: u32,
}

impl Health {
    pub fn get_value(&mut self) -> u32 {
        self.value
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Health {{ value: {} }}", self.value)
    }
}

#[derive(Default, Bundle, Clone, Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

pub fn get_characters(world: std::sync::Arc<World>) -> Array {
    world
        .entities()
        .meta
        .iter()
        .enumerate()
        .map(|(id, _)| Dynamic::from(id as u64))
        .collect::<Vec<Dynamic>>()
}

pub fn get_health(world: &mut std::sync::Arc<World>) -> Array {
    let world = std::sync::Arc::get_mut(world).unwrap();
    
    let mut query = world.query::<&Health>();
    query
        .iter(world)
        .map(|h| Dynamic::from(h.clone()))
        .collect::<Vec<Dynamic>>()
}

fn create_script_manager() -> Result<ScriptManager, Box<EvalAltResult>> {
    let mut script_manager = ScriptManager::new();

    script_manager.add_script(1, BehaviorType::Combat, &include_str!("../my_script.rhai"))?;

    script_manager
        .modify_engine()
        .register_type::<Health>()
        .register_get("value", Health::get_value)
        .register_fn("print", |h: &mut Health| h.to_string())
        .register_type::<Position>()
        .register_fn("get_characters", get_characters)
        .register_fn("get_health", get_health);
    //.register_iterator::<Vec<usize>>();

    Ok(script_manager)
}
