use bevy::prelude::*;

use crate::ComponentActorKind;

#[derive(Event, Debug)]
pub(crate) struct EventActorIsKilled {
    pub(crate) entity_killed: Entity,
    pub(crate) actor_type: ComponentActorKind,
}

impl EventActorIsKilled {
    pub(crate) fn new(entity_killed: Entity, actor_type: ComponentActorKind) -> Self {
        Self {
            entity_killed,
            actor_type,
        }
    }
}
