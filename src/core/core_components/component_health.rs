use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentHealth {
    pub(crate) current_health: u32,
}

impl ComponentHealth {
    pub(crate) fn new(current_health: u32) -> Self {
        Self { current_health }
    }
}
