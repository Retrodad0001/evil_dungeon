use bevy::prelude::*;

use crate::core::prelude::*;

#[derive(Bundle)]
pub(crate) struct KnightBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    name: Name,
}

impl KnightBundle {
    pub fn new(
        atlas_texture_handle: Handle<Image>,
        texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
        index: usize,
        spawn_position: Vec2,
    ) -> Self {
        Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture: atlas_texture_handle,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout_handle,
                    index,
                },
                transform: Transform::from_translation(Vec3::new(
                    spawn_position.x,
                    spawn_position.y,
                    DRAW_INDEX_Z_PLAYER,
                )),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::Center,
                    ..Default::default()
                },
                ..default()
            },
            name: Name::new("Knight (player)"),
        }
    }
}
