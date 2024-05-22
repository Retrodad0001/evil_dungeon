use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct ComponentCameraTag;

impl ComponentCameraTag {
    pub(crate) fn new() -> Self {
        Self
    }
}
