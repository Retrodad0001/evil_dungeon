use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct GameInfo {}

impl GameInfo {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
