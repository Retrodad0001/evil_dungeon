use bevy::prelude::*;

use crate::ComponentTileType;

#[derive(Component, Resource, Default)]
pub(crate) struct Room {
    pub(crate) top_left_start_pos: UVec2,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) tiles: Option<Vec<Vec<ComponentTileType>>>,
}

impl Room {
    pub(crate) fn new(top_left_start_pos: UVec2, room_width: u32, room_height: u32) -> Self {
        Self {
            top_left_start_pos,
            width: room_width,
            height: room_height,
            tiles: None,
        }
    }
}
