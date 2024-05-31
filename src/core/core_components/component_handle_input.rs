use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct ComponentHandleInput;

impl ComponentHandleInput {
    pub(crate) fn new() -> Self {
        Self
    }
}
