use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug, PartialEq, Clone, Copy)]
#[reflect(Resource)]
pub(crate) enum ComponentAiAction {
    #[default]
    Idle,
    Wandering,
    Chasing,
    AttackMelee,
    AttackingWithSpawningEnemies,
    Fleeing,
}
