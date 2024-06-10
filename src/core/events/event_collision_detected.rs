use bevy::prelude::*;

use crate::ComponentActorKind;

#[derive(Event, Debug)]
pub(crate) struct EventCollisionDetected {
    pub(crate) entity_a_actor_kind: ComponentActorKind,
    pub(crate) entity_b_actor_kind: ComponentActorKind,
    pub(crate) entity_a: Entity,
    pub(crate) entity_b: Entity,
}

impl EventCollisionDetected {
    pub(crate) fn new(
        entity_a_actor_kind: ComponentActorKind,
        entity_b_actor_kind: ComponentActorKind,
        entity_a: Entity,
        entity_b: Entity,
    ) -> Self {
        Self {
            entity_a_actor_kind,
            entity_b_actor_kind,

            entity_a,
            entity_b,
        }
    }
}
