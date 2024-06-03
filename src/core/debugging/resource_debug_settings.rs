use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub(crate) struct ResourceDebugSettings {
    pub(crate) show_debug_info: bool,
}

impl ResourceDebugSettings {
    pub(crate) fn new() -> Self {
        Self {
            show_debug_info: true,
        }
    }
}
