use super::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug)]

pub(crate) struct ComponentAnimationClip {
    pub(crate) animation_kind: ComponentAnimationClipKind,
    pub(crate) animation_frames: Vec<i32>,
    pub(crate) time_between_frames_sec: f32,
    pub(crate) should_flip_x: bool,
}

impl ComponentAnimationClip {
    pub(crate) fn new(
        animation_kind: ComponentAnimationClipKind,
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
