#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TexturePackerJsonDTO {
    pub(crate) meta: Meta,
    pub(crate) frames: Vec<DtoLine>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct DtoLine {
    pub(crate) filename: String, //frame_filename
    pub(crate) frame: Frame,
    pub(crate) rotated: bool,
    pub(crate) trimmed: bool,
    pub(crate) spriteSourceSize: SpriteSourceSize,
    pub(crate) sourceSize: SourceSize,
    pub(crate) pivot: Pivot,
}

//TOD move to separate files
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Meta {
    pub(crate) size: Size,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Size {
    pub(crate) w: u32,
    pub(crate) h: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Frame {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SpriteSourceSize {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) w: i32,
    pub(crate) h: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SourceSize {
    pub(crate) w: i32,
    pub(crate) h: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Pivot {
    pub(crate) x: f32,
    pub(crate) y: f32,
}
