use std::fs;

use crate::tiw_asset_management::prelude::*;

pub(crate) fn create_dto_from_json_file() -> TexturePackerJsonDTO {
    let data: String = fs::read_to_string("assets/atlas.json").expect("Unable to read file");

    let json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON does not have correct format.");

    let dto: TexturePackerJsonDTO = serde_json::from_str(json.to_string().as_str()).unwrap();

    debug_assert!(
        dto.meta.size.w > 0 && dto.meta.size.h > 0,
        "Texture packer json file has invalid dimensions."
    );

    debug_assert!(
        !dto.frames.is_empty(),
        "No frames found in the texture packer json file."
    );

    dto
}
