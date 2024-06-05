use bevy::prelude::*;

use crate::TileType;

use super::prelude::MapGenerationInput;

#[derive(Component, Resource, Default)]
pub(crate) struct TiwTileMap {
    pub(crate) map_width: u32,
    pub(crate) map_height: u32,
    pub(crate) floor_level: Vec<Vec<TileType>>,
}

impl TiwTileMap {
    pub(crate) fn new() -> Self {
        Self {
            map_width: 0,
            map_height: 0,
            floor_level: Vec::new(),
        }
    }

    pub(crate) fn generate_level(&mut self, map_generation_input: MapGenerationInput) {
        self.map_width = map_generation_input.map_width;
        self.map_height = map_generation_input.map_height;

        let mut new_floor_map: Vec<Vec<TileType>> = Vec::new();

        //generate floor map
        for _y in 0..map_generation_input.map_height {
            let mut row: Vec<TileType> = Vec::new();
            for _x in 0..map_generation_input.map_width {
                row.push(TileType::Floor0);
            }
            new_floor_map.push(row);
        }

        //* temp: generate walls for testing collision
        new_floor_map[0][0] = TileType::MidWall;
        new_floor_map[0][1] = TileType::MidWall;
        new_floor_map[0][2] = TileType::MidWall;
        new_floor_map[0][3] = TileType::MidWall;
        new_floor_map[0][4] = TileType::MidWall;
        new_floor_map[3][3] = TileType::MidWall;
        new_floor_map[3][4] = TileType::MidWall;
        new_floor_map[3][5] = TileType::MidWall;
        new_floor_map[5][5] = TileType::MidWall;
        new_floor_map[6][7] = TileType::MidWall;

        self.floor_level = new_floor_map;
    }
}
