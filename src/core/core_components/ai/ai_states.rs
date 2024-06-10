use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug, PartialEq, Clone, Copy)]
#[reflect(Resource)]
pub(crate) enum AiState {
    #[default]
    Idle,
    Wandering,
    Chasing,
    Attacking,
    Dead,
    Fleeing,
}
