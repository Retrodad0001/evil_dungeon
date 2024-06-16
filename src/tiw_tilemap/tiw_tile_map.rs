use super::prelude::MapGenerationInput;
use crate::ComponentTileType;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Resource, Default)]
pub(crate) struct TiwTileMap {
    pub(crate) map_width: u32,
    pub(crate) map_height: u32,
    pub(crate) tile_map: Vec<Vec<ComponentTileType>>,
}

impl TiwTileMap {
    pub(crate) fn new() -> Self {
        Self {
            map_width: 0,
            map_height: 0,
            tile_map: Vec::new(),
        }
    }

    pub(crate) fn generate_level(&mut self, map_generation_input: MapGenerationInput) {
        self.map_width = map_generation_input.width;
        self.map_height = map_generation_input.height;

        let mut new_floor_map: Vec<Vec<ComponentTileType>> = Vec::new();

        //* generate rooms
        generate_room(map_generation_input, &mut new_floor_map); //

        self.tile_map = new_floor_map;
    }

    #[inline(always)]
    pub(crate) fn is_blocking_tile_at_location(&self, x: f32, y: f32) -> bool {
        let tile_type: ComponentTileType = self.get_tile_type_at_world_location(x, y);

        self.is_blocking_tile(&tile_type)
    }

    #[inline(always)]
    pub(crate) fn is_blocking_tile(&self, tile_type: &ComponentTileType) -> bool {
        match tile_type {
            ComponentTileType::MidWall => true,
            ComponentTileType::Floor0 => false,
        }
    }

    #[inline(always)]
    pub(crate) fn get_tile_type_at_world_location(&self, x: f32, y: f32) -> ComponentTileType {
        const TILE_WH: i32 = 16; //* i put it here, will never change and all tiles have same W H */
        let world_x: u32 = x as u32;
        let world_y: u32 = y as u32;

        let tile_x: u32 = world_x / TILE_WH as u32;
        let tile_y: u32 = world_y / TILE_WH as u32;

        self.tile_map[tile_y as usize][tile_x as usize]
    }

    #[inline(always)]
    pub(crate) fn get_world_position_from_tile_position(&self, tile_x: u32, tile_y: u32) -> Vec2 {
        const TILE_WH: f32 = 16.0; //* i put it here, will never change and all tiles have same W H */
        let world_x: f32 = tile_x as f32 * TILE_WH;
        let world_y: f32 = tile_y as f32 * TILE_WH;

        Vec2::new(world_x + TILE_WH / 2.0, world_y + TILE_WH / 2.0)
    }

    #[inline(always)]
    pub(crate) fn get_random_non_blocking_tile_world_position(&self) -> Vec2 {
        let mut grid_location_found: bool = false;

        let mut rng: ThreadRng = rand::thread_rng();
        let mut result: Vec2 = Vec2::new(2.0, 2.0);

        while !grid_location_found {
            let random_x: u32 = rng.gen_range(1..self.map_width - 1);
            let random_y: u32 = rng.gen_range(1..self.map_height - 1);
            let tile_type: ComponentTileType = self.tile_map[random_y as usize][random_x as usize];

            if !self.is_blocking_tile(&tile_type) {
                grid_location_found = true;
                result = self.get_world_position_from_tile_position(random_x, random_y);
            }
        }

        result
    }
}

fn generate_room(
    generation_input: MapGenerationInput,
    new_floor_map: &mut Vec<Vec<ComponentTileType>>,
) {
    for _y in 0..generation_input.height {
        let mut row: Vec<ComponentTileType> = Vec::new();
        for _x in 0..generation_input.width {
            row.push(ComponentTileType::Floor0);
        }
        new_floor_map.push(row);
    }

    //generate walls in room
    for y in 0..generation_input.height {
        for x in 0..generation_input.width {
            if x == 0 || x == generation_input.width - 1 {
                new_floor_map[y as usize][x as usize] = ComponentTileType::MidWall;
            }
            if y == 0 || y == generation_input.height - 1 {
                new_floor_map[y as usize][x as usize] = ComponentTileType::MidWall;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tile_type_at_world_location() {
        let mut tiw_tile_map: TiwTileMap = TiwTileMap::new();
        tiw_tile_map.map_width = 2;
        tiw_tile_map.map_height = 2;

        let mut new_floor_map: Vec<Vec<ComponentTileType>> = Vec::new();

        //generate floor map
        for _y in 0..tiw_tile_map.map_height {
            let mut row: Vec<ComponentTileType> = Vec::new();
            for _x in 0..tiw_tile_map.map_width {
                row.push(ComponentTileType::Floor0);
            }
            new_floor_map.push(row);
        }

        new_floor_map[0][0] = ComponentTileType::MidWall;

        tiw_tile_map.tile_map = new_floor_map;

        let world_x: f32 = 0.0;
        let world_y: f32 = 0.0;
        let tile_type00: ComponentTileType =
            tiw_tile_map.get_tile_type_at_world_location(world_x, world_y);
        assert_eq!(tile_type00, ComponentTileType::MidWall);

        let world_x: f32 = 1.0;
        let world_y: f32 = 1.0;
        let tile_type00: ComponentTileType =
            tiw_tile_map.get_tile_type_at_world_location(world_x, world_y);
        assert_eq!(tile_type00, ComponentTileType::MidWall);

        let world_x: f32 = 15.0;
        let world_y: f32 = 15.0;
        let tile_type00: ComponentTileType =
            tiw_tile_map.get_tile_type_at_world_location(world_x, world_y);
        assert_eq!(tile_type00, ComponentTileType::MidWall);

        let world_x: f32 = 16.0;
        let world_y: f32 = 16.0;
        let tile_type00: ComponentTileType =
            tiw_tile_map.get_tile_type_at_world_location(world_x, world_y);
        assert_eq!(tile_type00, ComponentTileType::Floor0);

        let world_x: f32 = 17.0;
        let world_y: f32 = 17.0;
        let tile_type00: ComponentTileType =
            tiw_tile_map.get_tile_type_at_world_location(world_x, world_y);
        assert_eq!(tile_type00, ComponentTileType::Floor0);
    }
}
