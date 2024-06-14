use bevy::prelude::*;

use crate::core::prelude::*;

use super::component_tile_type;

#[derive(Bundle)]
pub(crate) struct FloorBundle {
    name: Name,
    sprite_sheet_bundle: SpriteSheetBundle,
    tile_type: component_tile_type::ComponentTileType,
}

impl FloorBundle {
    pub fn new(
        atlas_texture_handle: Handle<Image>,
        texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
        index: usize,
        spawn_position: Vec2,
        tile_type: component_tile_type::ComponentTileType,
    ) -> Self {
        Self {
            name: Name::new("Floor tile"),
            sprite_sheet_bundle: SpriteSheetBundle {
                texture: atlas_texture_handle,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout_handle,
                    index,
                },
                transform: Transform::from_translation(Vec3::new(
                    spawn_position.x,
                    spawn_position.y,
                    DRAW_INDEX_Z_TILEMAP_FLOOR,
                )),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::TopLeft,
                    flip_x: false,
                    ..Default::default()
                },
                ..default()
            },
            tile_type,
        }
    }
}
