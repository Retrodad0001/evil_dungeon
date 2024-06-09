use bevy::prelude::*;

#[derive(Event, Debug)]
pub(crate) struct EventActorIsKilled {
    pub(crate) entity_killed: Entity,
}

impl EventActorIsKilled {
    pub(crate) fn new(entity_killed: Entity) -> Self {
        Self { entity_killed }
    }
}
