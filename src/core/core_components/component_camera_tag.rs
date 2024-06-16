use bevy::prelude::*;

#[derive(Component, Debug)]

pub(crate) struct ComponentCameraTag;

impl ComponentCameraTag {
    pub(crate) fn new() -> Self {
        Self
    }
}
