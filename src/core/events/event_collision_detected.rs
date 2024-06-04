use bevy::prelude::*;

#[derive(Event, Debug)]
pub(crate) struct EventCollisionDetected {
    pub(crate) entity_a: Entity,
    pub(crate) entity_b: Entity,
}

impl EventCollisionDetected {
    pub(crate) fn new(entity_a: Entity, entity_b: Entity) -> Self {
        Self { entity_a, entity_b }
    }
}
