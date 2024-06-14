use bevy::prelude::*;

use crate::core::prelude::*;

#[derive(Bundle)]
pub(crate) struct BigZombieBundle {
    name: Name,
    actor_kind: ComponentActorKind,
    sprite_sheet_bundle: SpriteSheetBundle,
    movement: ComponentCanMove,
    animation: ComponentCanAnimate,
    health: ComponentHasHealth,
    collision: ComponentCanCollide,
    damage_dealer: ComponentCanDealDamage,
    ai_state: ComponentAI,
}

impl BigZombieBundle {
    pub fn new(
        atlas_texture_handle: Handle<Image>,
        texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
        index: usize,
        spawn_position: Vec2,
        start_health: i32,
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
            movement: ComponentCanMove::new(20.0),
            animation: ComponentCanAnimate::new(ComponentAnimationClipKind::KnightMoving),
            health: ComponentHasHealth::new(start_health),
            collision: ComponentCanCollide::new(
                Vec2::new(0.5, -5.0),
                12.0,
                15.0,
                ComponentActorKind::BigZombie,
                vec![ComponentActorKind::PlayerKnight],
            ),
            damage_dealer: ComponentCanDealDamage::new(25),
            ai_state: ComponentAI::new(AiState::Idle, 3.0, 80.0, 8.0),
        }
    }
}
