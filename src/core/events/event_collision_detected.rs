use bevy::prelude::*;

use crate::ComponentActorKind;

#[derive(Event, Debug)]
pub(crate) struct EventCollisionDetected {
    pub(crate) entity_a_actor_kind: ComponentActorKind,
    pub(crate) entity_b_actor_kind: ComponentActorKind,
}

impl EventCollisionDetected {
    pub(crate) fn new(
        entity_a_actor_kind: ComponentActorKind,
        entity_b_actor_kind: ComponentActorKind,
    ) -> Self {
        Self {
            entity_a_actor_kind,
            entity_b_actor_kind,
        }
    }
}
