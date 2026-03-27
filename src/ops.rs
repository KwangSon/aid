use crate::data::{Entity, Main};
use crate::geom::{Circle, Line};

pub struct Context<'a> {
    pub main: &'a mut Main,
    pub active_scene_id: u64,
}

pub trait Operator {
    fn poll(&self, _context: &Context) -> bool {
        true
    }
    fn execute(&self, context: &mut Context);
}

pub struct AddLineOperator {
    pub name: String,
    pub line: Line,
}

impl AddLineOperator {
    pub fn new(name: &str, line: Line) -> Self {
        Self {
            name: name.to_string(),
            line,
        }
    }
}

impl Operator for AddLineOperator {
    fn poll(&self, context: &Context) -> bool {
        context.main.scenes.contains_key(&context.active_scene_id)
    }

    fn execute(&self, context: &mut Context) {
        let id = context.main.generate_id();
        let entity = Entity::new_line(id, &self.name, self.line);
        context.main.add_entity(entity);
        if let Some(scene) = context.main.get_scene_mut(context.active_scene_id) {
            scene.add_entity(id);
        }
    }
}

pub struct AddCircleOperator {
    pub name: String,
    pub circle: Circle,
}

impl AddCircleOperator {
    pub fn new(name: &str, circle: Circle) -> Self {
        Self {
            name: name.to_string(),
            circle,
        }
    }
}

impl Operator for AddCircleOperator {
    fn poll(&self, context: &Context) -> bool {
        context.main.scenes.contains_key(&context.active_scene_id)
    }

    fn execute(&self, context: &mut Context) {
        let id = context.main.generate_id();
        let entity = Entity::new_circle(id, &self.name, self.circle);
        context.main.add_entity(entity);
        if let Some(scene) = context.main.get_scene_mut(context.active_scene_id) {
            scene.add_entity(id);
        }
    }
}
