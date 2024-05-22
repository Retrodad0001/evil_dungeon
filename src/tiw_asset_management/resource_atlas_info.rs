use bevy::prelude::*;
use std::collections::HashMap;

use crate::TexturePackerJsonDTO;

#[derive(Resource, Debug, Default)]
pub(crate) struct ResourceAtlasInfo {
    pub(crate) layout_dimensions: Vec2,
    pub(crate) file_name_index_mappings: HashMap<usize, String>,
    pub(crate) texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
    pub(crate) atlas_texture_handle: Handle<Image>,
}

impl ResourceAtlasInfo {
    pub(crate) fn new() -> Self {
        ResourceAtlasInfo {
            layout_dimensions: Vec2::new(0.0, 0.0),
            file_name_index_mappings: HashMap::new(),
            texture_atlas_layout_handle: Handle::default(),
            atlas_texture_handle: Handle::default(),
        }
    }

    pub(crate) fn initialize(
        &mut self,
        texture_packer_dto: TexturePackerJsonDTO,
    ) -> TextureAtlasLayout {
        self.file_name_index_mappings.clear(); //when reloading the assets

        self.layout_dimensions = Vec2::new(
            texture_packer_dto.meta.size.w as f32,
            texture_packer_dto.meta.size.h as f32,
        );

        //setup the layout of the buildin bevy texture atlas
        let mut texture_atlas_layout: TextureAtlasLayout =
            TextureAtlasLayout::new_empty(self.layout_dimensions);

        for dto_line in texture_packer_dto.frames.into_iter().enumerate() {
            let frame_width = dto_line.1.frame.w;
            let frame_height = dto_line.1.frame.h;

            let top_x: f32 = dto_line.1.frame.x;
            let top_y: f32 = dto_line.1.frame.y;
            let bottom_x: f32 = dto_line.1.frame.x + frame_width;
            let bottom_y: f32 = dto_line.1.frame.y + frame_height;

            let atlas_region = Rect {
                min: Vec2::new(top_x, top_y),
                max: Vec2::new(bottom_x, bottom_y),
            };

            let atlas_index: usize = texture_atlas_layout.add_texture(atlas_region);
            let filename: String = dto_line.1.filename.clone();
            debug!("Adding frame: {} at index: {}", filename, atlas_index);
            self.file_name_index_mappings.insert(atlas_index, filename);
        }

        texture_atlas_layout
    }

    pub(crate) fn get_bevy_atlas_index_by_file_name(&self, file_name_needed: &str) -> usize {
        debug!("Looking for frame name: {}", file_name_needed);

        for (index, file_name) in self.file_name_index_mappings.iter() {
            if file_name == file_name_needed {
                return *index;
            }
        }

        panic!("Frame name not found in texture packer info.");
    }
}
