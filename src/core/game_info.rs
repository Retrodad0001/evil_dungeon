use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct GameInfo {
    //like enalble show AI paths
}

impl GameInfo {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
