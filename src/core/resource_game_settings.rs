use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct ResourceGameSettings {}

impl ResourceGameSettings {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
