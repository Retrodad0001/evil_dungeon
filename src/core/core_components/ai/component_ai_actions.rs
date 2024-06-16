use bevy::prelude::*;

#[derive(Component, Default, Debug, PartialEq, Clone, Copy)]
pub(crate) enum ComponentAiAction {
    #[default]
    Idle,
    Wandering,
    Chasing,
    AttackMelee,
    AttackingWithSpawningEnemies,
    Fleeing,
}
