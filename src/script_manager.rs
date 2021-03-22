use std::collections::HashMap;

use rhai::{AST, Engine, EvalAltResult, OptimizationLevel};

pub struct ScriptManager{
    engine: Engine,
    asts: HashMap<i32, HashMap<BehaviorType, AST>>,
}

impl ScriptManager {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        engine.set_optimization_level(OptimizationLevel::Full);

        ScriptManager {
            engine,
            asts: HashMap::new(),
        }
    }

    pub fn get_engine(&self) -> &Engine {
        &self.engine
    }

    pub fn modify_engine(&mut self) -> &mut Engine {
        &mut self.engine
    }

    pub fn add_script(
        &mut self,
        entity_id: i32,
        behavior_type: BehaviorType,
        script: &str,
    ) -> Result<(), Box<EvalAltResult>> {
        let entity = if let Some(entity) = self.asts.get_mut(&entity_id) {
            entity
        } else {
            let entity_hash = HashMap::new();
            self.asts.insert(entity_id, entity_hash);

            self.asts.get_mut(&entity_id).unwrap()
        };

        let ast = self.engine.compile(script)?;

        entity.insert(behavior_type, ast);

        Ok(())
    }

    pub fn get_script(&self, entity_id: i32, behavior_type: BehaviorType) -> Option<&AST> {
        if let Some(entity) = self.asts.get(&entity_id) {
            if let Some(behavior) = entity.get(&behavior_type) {
                return Some(behavior);
            }
        }

        None
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum BehaviorType {
    Combat,
}
