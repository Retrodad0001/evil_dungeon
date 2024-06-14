use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub(crate) enum ComponentTileType {
    Floor0 = 0,
    MidWall = 1,
}
