use bevy::prelude::*;

use crate::TiWAI;

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentAIFlee;

impl ComponentAIFlee {
    pub(crate) fn new() -> Self {
        Self
    }
}
impl TiWAI for ComponentAIFlee {
    fn process(&self) {}
}
