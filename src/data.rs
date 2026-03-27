use crate::geom::{Circle, Line};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct IDData {
    pub name: String,
    pub id: u64,
}

impl IDData {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityData {
    Line(Line),
    Circle(Circle),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub id: IDData,
    pub data: EntityData,
}

impl Entity {
    pub fn new_line(id: u64, name: &str, line: Line) -> Self {
        Self {
            id: IDData::new(id, name),
            data: EntityData::Line(line),
        }
    }

    pub fn new_circle(id: u64, name: &str, circle: Circle) -> Self {
        Self {
            id: IDData::new(id, name),
            data: EntityData::Circle(circle),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub id: IDData,
    pub entity_ids: Vec<u64>,
}

impl Scene {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id: IDData::new(id, name),
            entity_ids: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity_id: u64) {
        if !self.entity_ids.contains(&entity_id) {
            self.entity_ids.push(entity_id);
        }
    }
}

pub struct Main {
    pub entities: HashMap<u64, Entity>,
    pub scenes: HashMap<u64, Scene>,
    next_id: u64,
}

impl Main {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            scenes: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id.id, entity);
    }

    pub fn remove_entity(&mut self, id: u64) {
        self.entities.remove(&id);
        // Also remove from all scenes
        for scene in self.scenes.values_mut() {
            scene.entity_ids.retain(|&eid| eid != id);
        }
    }

    pub fn update_entity_data(&mut self, id: u64, data: EntityData) {
        if let Some(entity) = self.entities.get_mut(&id) {
            entity.data = data;
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.insert(scene.id.id, scene);
    }

    pub fn get_entity(&self, id: u64) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_scene_mut(&mut self, id: u64) -> Option<&mut Scene> {
        self.scenes.get_mut(&id)
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}
