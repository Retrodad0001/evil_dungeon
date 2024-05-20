use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct TexturePackerInfo {}

impl TexturePackerInfo {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
