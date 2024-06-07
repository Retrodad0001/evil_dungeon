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

    pub(crate) fn is_blocking_tile_at_location(&self, x: f32, y: f32, tile_wh: i32) -> bool {
        let tile_type: TileType = self.get_tile_type_at_location(x, y, tile_wh);

        match tile_type {
            TileType::MidWall => true,
            TileType::Floor0 => false,
        }
    }

    pub(crate) fn get_tile_type_at_location(&self, x: f32, y: f32, tile_wh: i32) -> TileType {
        let x = x as u32;
        let y = y as u32;

        let x = x / tile_wh as u32;
        let y = y / tile_wh as u32;

        let result: &TileType = self
            .floor_level
            .get(y as usize)
            .unwrap()
            .get(x as usize)
            .unwrap();

        *result
    }
}
