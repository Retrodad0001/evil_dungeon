use super::prelude::MapGenerationInput;
use crate::TileType;
use bevy::prelude::*;

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

        //generate room
        for _y in 0..map_generation_input.map_height {
            let mut row: Vec<TileType> = Vec::new();
            for _x in 0..map_generation_input.map_width {
                row.push(TileType::Floor0);
            }
            new_floor_map.push(row);
        }

        //generate walls
        for y in 0..map_generation_input.map_height {
            for x in 0..map_generation_input.map_width {
                if x == 0 || x == map_generation_input.map_width - 1 {
                    new_floor_map[y as usize][x as usize] = TileType::MidWall;
                }
                if y == 0 || y == map_generation_input.map_height - 1 {
                    new_floor_map[y as usize][x as usize] = TileType::MidWall;
                }
            }
        }

        self.floor_level = new_floor_map;
    }

    pub(crate) fn is_blocking_tile_at_location(&self, x: f32, y: f32) -> bool {
        let tile_type: TileType = self.get_tile_type_at_location(x, y);

        match tile_type {
            TileType::MidWall => true,
            TileType::Floor0 => false,
        }
    }

    pub(crate) fn get_tile_type_at_location(&self, x: f32, y: f32) -> TileType {
        const TILE_WH: i32 = 16; //* i put it here, will never change and all tiles have same W H */
        let world_x: u32 = x as u32;
        let world_y: u32 = y as u32;

        let tile_x: u32 = world_x / TILE_WH as u32;
        let tile_y: u32 = world_y / TILE_WH as u32;

        self.floor_level[tile_y as usize][tile_x as usize]
    }

    pub(crate) fn get_world_position_from_tile_position(&self, tile_x: u32, tile_y: u32) -> Vec2 {
        const TILE_WH: f32 = 16.0; //* i put it here, will never change and all tiles have same W H */
        let world_x: f32 = tile_x as f32 * TILE_WH;
        let world_y: f32 = tile_y as f32 * TILE_WH;

        Vec2::new(world_x + TILE_WH / 2.0, world_y + TILE_WH / 2.0)
    }

    pub(crate) fn get_random_non_blocking_tile_position(&self) -> Vec2 {
        const RANDOM_X: u32 = 2;
        const RANDOM_Y: u32 = 2;
        //TODO implement get_random_non_blocking_tile_position
        self.get_world_position_from_tile_position(RANDOM_X, RANDOM_Y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tile_type_at_location() {
        let mut tiw_tile_map: TiwTileMap = TiwTileMap::new();
        tiw_tile_map.map_width = 2;
        tiw_tile_map.map_height = 2;

        let mut new_floor_map: Vec<Vec<TileType>> = Vec::new();

        //generate floor map
        for _y in 0..tiw_tile_map.map_height {
            let mut row: Vec<TileType> = Vec::new();
            for _x in 0..tiw_tile_map.map_width {
                row.push(TileType::Floor0);
            }
            new_floor_map.push(row);
        }

        new_floor_map[0][0] = TileType::MidWall;

        tiw_tile_map.floor_level = new_floor_map;

        let world_x: f32 = 0.0;
        let world_y: f32 = 0.0;
        let tile_type00: TileType = tiw_tile_map.get_tile_type_at_location(world_x, world_y);
        assert_eq!(tile_type00, TileType::MidWall);

        let world_x: f32 = 1.0;
        let world_y: f32 = 1.0;
        let tile_type00: TileType = tiw_tile_map.get_tile_type_at_location(world_x, world_y);
        assert_eq!(tile_type00, TileType::MidWall);

        let world_x: f32 = 15.0;
        let world_y: f32 = 15.0;
        let tile_type00: TileType = tiw_tile_map.get_tile_type_at_location(world_x, world_y);
        assert_eq!(tile_type00, TileType::MidWall);

        let world_x: f32 = 16.0;
        let world_y: f32 = 16.0;
        let tile_type00: TileType = tiw_tile_map.get_tile_type_at_location(world_x, world_y);
        assert_eq!(tile_type00, TileType::Floor0);

        let world_x: f32 = 17.0;
        let world_y: f32 = 17.0;
        let tile_type00: TileType = tiw_tile_map.get_tile_type_at_location(world_x, world_y);
        assert_eq!(tile_type00, TileType::Floor0);
    }
}
