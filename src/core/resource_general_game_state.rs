use crate::tiw_tilemap::prelude::*;
use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct ResourceGeneralGameState {
    pub(crate) tiw_tile_map: TiwTileMap,
}

impl ResourceGeneralGameState {
    pub(crate) fn new() -> Self {
        Self {
            tiw_tile_map: TiwTileMap::new(),
        }
    }
}
