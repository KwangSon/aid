use crate::data::{Entity, EntityData, Main};
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

pub struct MoveEntityOperator {
    pub entity_id: u64,
    pub dx: f64,
    pub dy: f64,
}

impl MoveEntityOperator {
    pub fn new(entity_id: u64, dx: f64, dy: f64) -> Self {
        Self { entity_id, dx, dy }
    }
}

impl Operator for MoveEntityOperator {
    fn poll(&self, context: &Context) -> bool {
        context.main.entities.contains_key(&self.entity_id)
    }

    fn execute(&self, context: &mut Context) {
        if let Some(entity) = context.main.get_entity(self.entity_id) {
            let new_data = match &entity.data {
                EntityData::Line(line) => EntityData::Line(line.translate(self.dx, self.dy)),
                EntityData::Circle(circle) => {
                    EntityData::Circle(circle.translate(self.dx, self.dy))
                }
            };
            context.main.update_entity_data(self.entity_id, new_data);
        }
    }
}

pub struct DeleteEntityOperator {
    pub entity_id: u64,
}

impl DeleteEntityOperator {
    pub fn new(entity_id: u64) -> Self {
        Self { entity_id }
    }
}

impl Operator for DeleteEntityOperator {
    fn poll(&self, context: &Context) -> bool {
        context.main.entities.contains_key(&self.entity_id)
    }

    fn execute(&self, context: &mut Context) {
        context.main.remove_entity(self.entity_id);
    }
}
