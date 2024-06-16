#[derive(Debug)]
pub(crate) struct MapGenerationInput {
    pub(crate) width: u32,  //TODO remove when rooms are implemented in generation
    pub(crate) height: u32, //TODO remove when rooms are implemented in generation
}

impl MapGenerationInput {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
