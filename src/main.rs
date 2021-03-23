mod script_manager;

use std::{fmt::Display, sync::{Arc, Mutex}, time::Instant};

use script_manager::{BehaviorType, ScriptManager};

use bevy_ecs::prelude::*;
use rhai::{Array, Dynamic, EvalAltResult, Map, Scope};

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

    scope.push("world", Arc::new(Mutex::new(world)));

    let script_manager = create_script_manager()?;

    let result = script_manager.get_script(1, BehaviorType::Combat).unwrap();

    let start = Instant::now();
    script_manager
        .get_engine()
        .eval_ast_with_scope(&mut scope, &result)?;
    let end = Instant::now();

    println!("{}", (end - start).as_millis());

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

impl Position {
    pub fn get_value(&mut self) -> Map {
        let mut map = Map::new();

        map.insert("x".into(), Dynamic::from(self.x));
        map.insert("y".into(), Dynamic::from(self.y));

        map
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl From<Position> for Dynamic {
    fn from(position: Position) -> Self {
        Dynamic::from(position)
    }
}

impl From<&Position> for Dynamic {
    fn from(position: &Position) -> Self {
        Dynamic::from(position.clone())
    }
}

impl From<Health> for Dynamic {
    fn from(health: Health) -> Self {
        Dynamic::from(health)
    }
}

impl From<&Health> for Dynamic {
    fn from(health: &Health) -> Self {
        Dynamic::from(health.clone())
    }
}

pub fn get_characters(world: Arc<Mutex<World>>) -> Array {
    world.lock().unwrap()
        .entities()
        .meta
        .iter()
        .enumerate()
        .map(|(id, _)| Dynamic::from(id as u64))
        .collect::<Vec<Dynamic>>()
}

pub fn get_health(world: &mut Arc<Mutex<World>>) -> Array {
    let mut world = world.lock().unwrap();

    let mut query = world.query::<&Health>();
    query
        .iter(&world)
        .map(|h| h.into())
        .collect::<Vec<Dynamic>>()
}

pub fn get_position(world: &mut Arc<Mutex<World>>) -> Array {
    let mut world = world.lock().unwrap();

    let mut query = world.query::<&Position>();
    query
        .iter(&world)
        .map(|p| p.into())
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
        .register_get("value", Position::get_value)
        .register_fn("print", |p: &mut Position| p.to_string())
        .register_fn("get_characters", get_characters)
        .register_fn("get_health", get_health)
        .register_fn("get_positions", get_position);

    Ok(script_manager)
}
