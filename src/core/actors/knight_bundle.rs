use bevy::prelude::*;

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
    ) -> Self {
        Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture: atlas_texture_handle,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout_handle,
                    index,
                },
                transform: Transform::from_scale(Vec3::splat(6.0)),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..Default::default()
                },
                ..default()
            },
            name: Name::new("Knight (player)"),
        }
    }
}
