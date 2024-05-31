use super::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct TiwAnimationClip {
    pub(crate) animation_kind: AnimationClipKind,
    pub(crate) animation_frames: Vec<i32>,
    pub(crate) time_between_frames_sec: f32,
    pub(crate) should_flip_x: bool,
}

impl TiwAnimationClip {
    pub(crate) fn new(
        animation_kind: AnimationClipKind,
        animation_frames: Vec<i32>,
        time_between_frames_sec: f32,
        should_flip_x: bool,
    ) -> Self {
        Self {
            animation_kind,
            animation_frames,
            time_between_frames_sec,
            should_flip_x,
        }
    }
}
