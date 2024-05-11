use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct DebugSettings {
    //like enalble show AI paths
}

impl DebugSettings {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
