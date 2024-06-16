use bevy::prelude::*;

#[derive(Component, Debug)]

pub(crate) struct ComponentHasHealth {
    pub(crate) current_health: i32,
    pub(crate) marked_as_dead: bool,
}

impl ComponentHasHealth {
    pub(crate) fn new(current_health: i32) -> Self {
        debug_assert!(
            current_health > 0,
            "initial current_health must be greater than 0"
        );

        Self {
            current_health,
            marked_as_dead: false,
        }
    }
}
