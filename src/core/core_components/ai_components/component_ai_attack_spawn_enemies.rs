use bevy::prelude::*;

use crate::TiWAI;

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentAIAttackSpawnEnemies;

impl ComponentAIAttackSpawnEnemies {
    pub(crate) fn new() -> Self {
        Self
    }
}
impl TiWAI for ComponentAIAttackSpawnEnemies {
    fn process(&self) {}
}
