use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) enum ComponentAnimationClipKind {
    #[default]
    ClipKnightIdle = 0,
    ClipKnightMoving = 1,
}
