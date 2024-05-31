use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) enum AnimationClipKind {
    #[default]
    ClipKnightIdle = 0,
    ClipKnightMoving = 1,
}
