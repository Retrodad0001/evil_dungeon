use bevy::prelude::*;

#[derive(Component, Debug)]

pub(crate) struct ComponentAnimationClip {
    pub(crate) animation_frames: Vec<u32>,
    pub(crate) time_between_frames_sec: f32,
}

impl ComponentAnimationClip {
    pub(crate) fn new(animation_frames: Vec<u32>, time_between_frames_sec: f32) -> Self {
        Self {
            animation_frames,
            time_between_frames_sec,
        }
    }
}
