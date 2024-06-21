use bevy::prelude::*;

#[derive(Component, Default, Debug, PartialEq, Clone)]
pub(crate) enum ComponentAiAction {
    #[default]
    Idle,
    Wandering,
    Chasing,
    AttackMelee,
}
