use std::fs;

use crate::{tiw_asset_management::prelude::*, KNIGHT_IDLE_0};

pub(crate) fn create_dto_from_json_file() -> TexturePackerJsonDTO {
    let data: String = fs::read_to_string("assets/atlas.json").expect("Unable to read file");

    let json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON does not have correct format.");

    let dto: TexturePackerJsonDTO = serde_json::from_str(json.to_string().as_str()).unwrap();

    let _knight_idle_0: &DtoLine = dto
        .frames
        .iter()
        .find(|dto_line| dto_line.filename == KNIGHT_IDLE_0)
        .expect("knight_idle_0 not found in texture packer json file. damn....");

    dto
}
