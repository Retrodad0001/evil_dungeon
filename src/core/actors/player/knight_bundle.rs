use bevy::prelude::*;

use crate::core::prelude::*;

#[derive(Bundle)]
pub(crate) struct KnightBundle {
    name: Name,
    player_tag: ComponentPlayerTag,
    actor_kind: ComponentActorKind,
    sprite_sheet_bundle: SpriteSheetBundle,
    movement: ComponentMovement,
    animation: ComponentAnimator,
    health: ComponentHealth,
    collision: ComponentCollision,
    damage_dealer: ComponentDealDamage,
}

impl KnightBundle {
    pub fn new(
        atlas_texture_handle: Handle<Image>,
        texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
        index: usize,
        spawn_position: Vec2,
        start_health: u32,
    ) -> Self {
        Self {
            name: Name::new("Knight (player)"),
            player_tag: ComponentPlayerTag::new(),
            actor_kind: ComponentActorKind::Knight,
            sprite_sheet_bundle: SpriteSheetBundle {
                texture: atlas_texture_handle,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout_handle,
                    index,
                },
                transform: Transform::from_translation(Vec3::new(
                    spawn_position.x,
                    spawn_position.y,
                    DRAW_INDEX_Z_PLAYER_ENEMIES,
                )),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::Center,
                    flip_x: false,
                    ..Default::default()
                },
                ..default()
            },
            movement: ComponentMovement::new(),
            animation: ComponentAnimator::new(ComponentAnimationClipKind::KnightIdle),
            health: ComponentHealth::new(start_health),
            collision: ComponentCollision::new(),
            damage_dealer: ComponentDealDamage::new(10),
        }
    }
}
