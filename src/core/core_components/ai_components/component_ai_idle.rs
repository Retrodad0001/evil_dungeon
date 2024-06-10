use bevy::prelude::*;

use crate::TiWAI;

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentAIIdle;

impl ComponentAIIdle {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl TiWAI for ComponentAIIdle {
    fn process(&self) {}
}
