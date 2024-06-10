use bevy::prelude::*;

use crate::TiWAI;

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentAIWandering;

impl ComponentAIWandering {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl TiWAI for ComponentAIWandering {
    fn process(&self) {}
}
