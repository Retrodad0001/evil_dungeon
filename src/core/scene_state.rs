use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) enum SceneState {
    #[default]
    Playing,
}
