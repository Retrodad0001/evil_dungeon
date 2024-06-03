use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentCameraTag;

impl ComponentCameraTag {
    pub(crate) fn new() -> Self {
        Self
    }
}
