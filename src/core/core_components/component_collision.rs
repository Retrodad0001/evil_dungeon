use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentCollision;

impl ComponentCollision {
    pub(crate) fn new() -> Self {
        Self
    }
}
