use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct ResourceDebugSettings {
    pub(crate) show_performance_stats: bool,
    pub(crate) show_pivot_points: bool,
}

impl ResourceDebugSettings {
    pub(crate) fn new() -> Self {
        Self {
            show_performance_stats: false,
            show_pivot_points: false,
        }
    }
}
