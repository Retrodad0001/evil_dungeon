use crate::tiw_tilemap::prelude::*;
use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct ResourceGeneralGameState {
    pub(crate) level: u32,
    pub(crate) score: u32,
    pub(crate) tiw_tile_map: TiwTileMap,
}

impl ResourceGeneralGameState {
    pub(crate) fn new() -> Self {
        Self {
            level: 1,
            score: 0,
            tiw_tile_map: TiwTileMap::new(),
        }
    }
}
