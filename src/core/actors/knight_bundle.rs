use bevy::prelude::*;

use crate::core::prelude::*;
use crate::tiw_animation::prelude::*;

#[derive(Bundle)]
pub(crate) struct KnightBundle {
    name: Name,
    actor_kind: ActorKind,
    sprite_sheet_bundle: SpriteSheetBundle,
    handle_input: ComponentHandleInput,
    movement: ComponentMovement,
    animation: ComponentAnimation,
}

impl KnightBundle {
    pub fn new(
        atlas_texture_handle: Handle<Image>,
        texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
        index: usize,
        spawn_position: Vec2,
    ) -> Self {
        Self {
            name: Name::new("Knight (player)"),
            actor_kind: ActorKind::Knight,
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
                    flip_x: false,
                    ..Default::default()
                },
                ..default()
            },
            handle_input: ComponentHandleInput::new(),
            movement: ComponentMovement::new(),
            animation: ComponentAnimation::new(AnimationClipKind::ClipKnightIdle),
        }
    }
}
