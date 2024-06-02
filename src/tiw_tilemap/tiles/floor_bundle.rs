use bevy::prelude::*;

use crate::core::prelude::*;

#[derive(Bundle)]
pub(crate) struct FloorBundle {
    name: Name,
    sprite_sheet_bundle: SpriteSheetBundle,
}

impl FloorBundle {
    pub fn new(
        atlas_texture_handle: Handle<Image>,
        texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
        index: usize,
        spawn_position: Vec2,
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
        }
    }
}
