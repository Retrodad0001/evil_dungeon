use bevy::prelude::*;

use crate::TiWAI;

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentAIChase;

impl ComponentAIChase {
    pub(crate) fn new() -> Self {
        Self
    }
}
impl TiWAI for ComponentAIChase {
    fn process(&self) {}
}
