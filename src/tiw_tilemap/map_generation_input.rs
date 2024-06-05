#[derive(Debug)]
pub(crate) struct MapGenerationInput {
    pub(crate) map_width: u32,
    pub(crate) map_height: u32,
}

impl MapGenerationInput {
    pub(crate) fn new(map_width: u32, map_height: u32) -> Self {
        Self {
            map_width,
            map_height,
        }
    }
}
