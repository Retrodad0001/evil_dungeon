use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct CameraTag;

impl CameraTag {
    pub(crate) fn new() -> Self {
        Self
    }
}
