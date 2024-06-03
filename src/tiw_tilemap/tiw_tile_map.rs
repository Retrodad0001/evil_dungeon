use bevy::prelude::*;

use super::prelude::MapGenerationInput;

#[derive(Component, Resource, Default)]
pub(crate) struct TiwTileMap {
    pub(crate) map_width: u32,
    pub(crate) map_height: u32,
    pub(crate) floor_level: Vec<Vec<u32>>,
}

impl TiwTileMap {
    pub(crate) fn new() -> Self {
        Self {
            map_width: 0,
            map_height: 0,
            floor_level: Vec::new(),
        }
    }

    pub(crate) fn generate_floor_map(&mut self, map_generation_input: MapGenerationInput) {
        self.map_width = map_generation_input.map_width;
        self.map_height = map_generation_input.map_height;

        let mut new_floor_map: Vec<Vec<u32>> = Vec::new();

        //generate floor map
        for _y in 0..map_generation_input.map_height {
            let mut row: Vec<u32> = Vec::new();
            for _x in 0..map_generation_input.map_width {
                row.push(map_generation_input.tile_floor_index);
            }
            new_floor_map.push(row);
        }

        self.floor_level = new_floor_map;
    }
}
