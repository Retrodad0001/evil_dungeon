use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) enum ScreenState {
    #[default]
    Playing,
}
