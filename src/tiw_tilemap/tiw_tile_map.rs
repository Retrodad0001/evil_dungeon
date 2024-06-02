use bevy::prelude::*;

#[derive(Component, Resource, Default)]
pub(crate) struct TiwTileMap {
    pub(crate) floor_level: Vec<Vec<u32>>,
}

impl TiwTileMap {
    pub(crate) fn new() -> Self {
        Self {
            floor_level: Vec::new(),
        }
    }

    pub(crate) fn generate_floor_map(&mut self, map_width: i32, map_height: i32) {
        let mut new_floor_map: Vec<Vec<u32>> = Vec::new();

        //generate floor map
        const TILE_FLOOR: u32 = 0;
        for _y in 0..map_height {
            let mut row: Vec<u32> = Vec::new();
            for _x in 0..map_width {
                row.push(TILE_FLOOR);
            }
            new_floor_map.push(row);
        }

        self.floor_level = new_floor_map;
    }
}
