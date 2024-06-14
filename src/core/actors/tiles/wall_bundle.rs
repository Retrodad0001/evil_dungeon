use bevy::prelude::*;

use crate::core::prelude::*;

#[derive(Bundle)]
pub(crate) struct WallBundle {
    name: Name,
    sprite_sheet_bundle: SpriteSheetBundle,
    actor_kind: ComponentActorKind,
    collision: ComponentCanCollide,
    tile_type: ComponentTileType,
}

impl WallBundle {
    pub fn new(
        atlas_texture_handle: Handle<Image>,
        texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
        index: usize,
        spawn_position: Vec2,
        tile_type: ComponentTileType,
    ) -> Self {
        Self {
            name: Name::new("Wall"),
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
            actor_kind: ComponentActorKind::Wall,
            collision: ComponentCanCollide::new(
                Vec2::new(8.0, -8.0),
                16.0,
                16.0,
                ComponentActorKind::Wall,
                Vec::new(),
            ),
            tile_type,
        }
    }
}
