use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) enum ScreenState {
    #[default]
    Playing,
}

//TODO when new sub-states is out in bevy add pause to playing
