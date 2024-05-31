use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub(crate) struct ResourceDebugSettings {
    pub(crate) show_debug_console: bool,
    pub(crate) show_pivot_points: bool,
}

impl ResourceDebugSettings {
    pub(crate) fn new() -> Self {
        Self {
            show_debug_console: true,
            show_pivot_points: true,
        }
    }
}
