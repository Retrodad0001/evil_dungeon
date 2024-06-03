use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) enum ComponentAnimationClipKind {
    #[default]
    KnightIdle = 0,
    KnightMoving = 1,
    BigZombieIdle = 2,
    BigZombieMoving = 3,
}
