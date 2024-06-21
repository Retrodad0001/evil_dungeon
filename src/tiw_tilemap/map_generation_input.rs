#[derive(Debug)]
pub(crate) struct MapGenerationInput {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl MapGenerationInput {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
