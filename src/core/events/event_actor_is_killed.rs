use bevy::prelude::*;

use crate::ComponentActorKind;

#[derive(Event, Debug)]
pub(crate) struct EventActorIsKilled {
    pub(crate) actor_kind: ComponentActorKind,
}

impl EventActorIsKilled {
    pub(crate) fn new(actor_kind: ComponentActorKind) -> Self {
        Self { actor_kind }
    }
}

impl std::fmt::Display for EventActorIsKilled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.actor_kind)
    }
}
