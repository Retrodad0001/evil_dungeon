use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentHasHealth {
    pub(crate) current_health: i32,
}

impl ComponentHasHealth {
    pub(crate) fn new(current_health: i32) -> Self {
        debug_assert!(
            current_health > 0,
            "initial current_health must be greater than 0"
        );

        Self { current_health }
    }
}
