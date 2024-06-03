#[derive(Debug)]
pub(crate) struct MapGenerationInput {
    pub(crate) map_width: u32,
    pub(crate) map_height: u32,
    pub(crate) tile_floor_index: u32, //can be vector with more options random
}

impl MapGenerationInput {
    pub(crate) fn new(map_width: u32, map_height: u32, tile_floor_index: u32) -> Self {
        Self {
            map_width,
            map_height,
            tile_floor_index,
        }
    }
}
