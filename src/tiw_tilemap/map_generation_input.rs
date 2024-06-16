#[derive(Debug)]
pub(crate) struct MapGenerationInput {
    pub(crate) width: u32,  //TODO remove when rooms are implemented
    pub(crate) height: u32, //TODO remove when rooms are implemented
    pub(crate) total_rooms: u32,
    pub(crate) room_min_size: u32,
    pub(crate) room_max_size: u32,
    pub(crate) bos_rooms: u32,
}

impl MapGenerationInput {
    pub(crate) fn new(
        width: u32,
        height: u32,
        total_rooms: u32,
        room_min_size: u32,
        room_max_size: u32,
        bos_rooms: u32,
    ) -> Self {
        Self {
            width,
            height,
            total_rooms,
            room_min_size,
            room_max_size,
            bos_rooms,
        }
    }
}
