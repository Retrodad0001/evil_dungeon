use bevy::prelude::*;

#[derive(Resource, Default)]
pub(crate) struct ResourceDebugSettings {
    pub(crate) show_pivot_points: bool,
    pub(crate) game_version_number: String,
}

impl ResourceDebugSettings {
    pub(crate) fn new(game_version_number: String) -> Self {
        Self {
            game_version_number,
            show_pivot_points: true,
        }
    }
}
