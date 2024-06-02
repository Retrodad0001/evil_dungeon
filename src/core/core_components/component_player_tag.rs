use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentPlayerTag;

impl ComponentPlayerTag {
    pub(crate) fn new() -> Self {
        Self
    }
}
