use bevy::prelude::*;

use crate::{AiState, ComponentActorKind};

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentAI {
    current_state: AiState,
}

impl ComponentAI {
    pub(crate) fn new(start_ai_state: AiState) -> Self {
        Self {
            current_state: start_ai_state,
        }
    }

    pub(crate) fn determine_new_state(&self, actor_kind: ComponentActorKind) {
        match actor_kind {
            ComponentActorKind::PlayerKnight | ComponentActorKind::Wall => {
                //* ignore, this actor kind has no AI
            }
            ComponentActorKind::BigZombie => {
                // Big Zombie AI
            }
        }
    }
}
