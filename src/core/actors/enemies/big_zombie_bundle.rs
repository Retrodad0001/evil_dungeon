use bevy::prelude::*;

use crate::core::prelude::*;

#[derive(Bundle)]
pub(crate) struct BigZombieBundle {
    name: Name,
    actor_kind: ComponentActorKind,
    sprite_sheet_bundle: SpriteSheetBundle,
    movement: ComponentMovement,
    animation: ComponentAnimator,
    health: ComponentHealth,
    collision: ComponentCollision,
    damage_dealer: ComponentDealDamage,
}

impl BigZombieBundle {
    pub fn new(
        atlas_texture_handle: Handle<Image>,
        texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
        index: usize,
        spawn_position: Vec2,
        start_health: u32,
    ) -> Self {
        Self {
            name: Name::new("Big Zombie"),
            actor_kind: ComponentActorKind::BigZombie,
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
            animation: ComponentAnimator::new(ComponentAnimationClipKind::KnightMoving),
            health: ComponentHealth::new(start_health),
            collision: ComponentCollision::new(),
            damage_dealer: ComponentDealDamage::new(25),
        }
    }
}
