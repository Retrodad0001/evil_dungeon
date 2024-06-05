use crate::core::prelude::*;
use crate::tiw_asset_management::prelude::*;
use crate::tiw_tilemap::prelude::MapGenerationInput;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use super::events::prelude::EventCollisionDetected;
use super::resource_general_game_state;

pub(crate) fn load_assets(
    bevy_asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_packer_info: ResMut<TexturePackerAtlasInfo>,
) {
    debug!("start - load_assets");

    let atlas_dto: TexturePackerJsonDTO = create_dto_from_json_file();
    let texture_atlas_layout: TextureAtlasLayout =
        texture_packer_info.setup_bevy_spite_atlas(atlas_dto);
    let texture_atlas_layout_handle: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(texture_atlas_layout);
    texture_packer_info.texture_atlas_layout_handle = texture_atlas_layout_handle;

    let atlas_texture: Handle<Image> = bevy_asset_server.load("sprites/atlas.png");
    texture_packer_info.atlas_texture_handle = atlas_texture;

    debug!("end - load_assets");
}

pub(crate) fn setup_animations(
    resource_atlas_info: ResMut<TexturePackerAtlasInfo>,
    mut resource_animation_info: ResMut<ResourceAnimationInfo>,
) {
    debug!("start - setup_animations");
    resource_animation_info.add_animation_clips(&resource_atlas_info);
    debug!("end - setup animations");
}

pub(crate) fn setup_camera(mut commands: Commands) {
    debug!("start - setup_camera");
    commands.spawn(TiWCamera2dBundle::new());
    debug!("end - setup_camera");
}

pub(crate) fn new_level(
    mut commands: Commands,
    atlas_info: Res<TexturePackerAtlasInfo>,
    mut resource_game_state: ResMut<resource_general_game_state::ResourceGeneralGameState>,
) {
    debug!("start - new_level");

    //generate floor map
    let map_generation_input = MapGenerationInput::new(20, 15);

    resource_game_state
        .tiw_tile_map
        .generate_level(map_generation_input);

    const TILE_SIZE_XY: f32 = 16.0;

    for y in 0..resource_game_state.tiw_tile_map.map_height {
        for x in 0..resource_game_state.tiw_tile_map.map_width {
            let tile: TileType =
                resource_game_state.tiw_tile_map.floor_level[y as usize][x as usize];

            match tile {
                TileType::Floor0 => {
                    let atlas_index_floor: usize =
                        atlas_info.get_bevy_atlas_index_by_file_name(FLOOR_0);
                    commands.spawn(FloorBundle::new(
                        atlas_info.atlas_texture_handle.clone(),
                        atlas_info.texture_atlas_layout_handle.clone(),
                        atlas_index_floor,
                        Vec2::new(x as f32 * TILE_SIZE_XY, y as f32 * TILE_SIZE_XY),
                    ));
                }
                TileType::MidWall => {
                    let atlas_index_wall: usize =
                        atlas_info.get_bevy_atlas_index_by_file_name(WALL_MID);
                    commands.spawn(WallBundle::new(
                        atlas_info.atlas_texture_handle.clone(),
                        atlas_info.texture_atlas_layout_handle.clone(),
                        atlas_index_wall,
                        Vec2::new(x as f32 * TILE_SIZE_XY, y as f32 * TILE_SIZE_XY),
                    ));
                }
            }
        }
    }

    //spawn player in level
    let index_knight_idle: usize = atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_0);
    commands.spawn(KnightBundle::new(
        atlas_info.atlas_texture_handle.clone(),
        atlas_info.texture_atlas_layout_handle.clone(),
        index_knight_idle,
        Vec2::new(200.0, 200.0),
        100,
    ));

    //spawn enemies in level
    let index_big_zombie_idle: usize =
        atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_IDLE_0);
    commands.spawn(BigZombieBundle::new(
        atlas_info.atlas_texture_handle.clone(),
        atlas_info.texture_atlas_layout_handle.clone(),
        index_big_zombie_idle,
        Vec2::new(140.0, 140.0),
        50,
    ));

    debug!("end - new_level");
}

pub(crate) fn calculate_direction_for_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut ComponentMovement, &ComponentPlayerTag)>,
) {
    let mut direction: Vec2 = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x = -1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x = 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y = 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y = -1.0;
    }

    let mut player: (Mut<ComponentMovement>, &ComponentPlayerTag) = player_query.single_mut();

    //TODO if in that direction is a collider, do not move (test with ray cast)
    let is_blocked_by_collider: bool = false;

    if is_blocked_by_collider {
        player.0.direction = Vec2::ZERO
    } else {
        player.0.direction = direction.normalize_or_zero();
    }
}

pub(crate) fn do_fancy_ai_for_enemies() {}

pub(crate) fn calculate_direction_for_enemies() {}

pub(crate) fn animate_all(
    mut animation_entities_query: Query<(
        &ComponentActorKind,
        &ComponentMovement,
        &mut ComponentAnimator,
        &mut TextureAtlas,
        &mut Sprite,
    )>,
    animation_info: Res<ResourceAnimationInfo>,
    time: Res<Time>,
) {
    let delta_time: f32 = time.delta_seconds();

    for (actor_kind, movement, mut animation, mut texture_atlas, mut sprite) in
        animation_entities_query.iter_mut()
    {
        let atlas_index: i32 = animation
            .determine_current_atlas_index_for_animation(
                actor_kind,
                &movement.direction,
                delta_time,
                &animation_info,
            )
            .unwrap();

        texture_atlas.index = atlas_index as usize;

        let should_flip_x: bool = movement.direction.x < 0.0;
        sprite.flip_x = should_flip_x;
    }
}

pub(crate) fn calculate_velocity_for_all(
    mut movement_entities_query: Query<(&mut Transform, &mut ComponentMovement)>,
    time: Res<Time>,
) {
    let delta_time: f32 = time.delta_seconds();

    for (mut transform, mut movement) in movement_entities_query.iter_mut() {
        movement.calculate_velocity(&delta_time);
        transform.translation += Vec3::new(
            movement.current_velocity.x,
            movement.current_velocity.y,
            0.0,
        );
    }
}

pub(crate) fn physics_determine_collision_for_all(
    collision_entities_query: Query<(
        Entity,
        &Transform,
        &mut ComponentCollision,
        &ComponentActorKind,
    )>,
    mut event_collision_detected: EventWriter<EventCollisionDetected>,
) {
    for (entity_a, transform_a, collision_a, actor_kind_a) in collision_entities_query.iter() {
        for (entity_b, transform_b, collision_b, actor_kind_b) in collision_entities_query.iter() {
            let should_ignore_collision_processing: bool = collision_a
                .should_ignore_collision_processing(entity_a, entity_b, collision_a, collision_b);

            if should_ignore_collision_processing {
                continue;
            }

            let entity_a_bounds: Aabb2d = collision_a.get_aabb2d_bounds(&transform_a.translation);
            let entity_b_bounds: Aabb2d = collision_b.get_aabb2d_bounds(&transform_b.translation);
            let has_collided: bool = entity_a_bounds.intersects(&entity_b_bounds);

            if has_collided {
                event_collision_detected.send(EventCollisionDetected::new(
                    entity_a,
                    entity_b,
                    *actor_kind_a,
                    *actor_kind_b,
                ));
                // debug!("collision between {:?} and {:?}", name_a, name_b);
            }
        }
    }
}

pub(crate) fn handle_health_when_event_collision_for_all(
    mut event_collision_detected: EventReader<EventCollisionDetected>,
    mut event_Actor_is_killed: EventWriter<EventActorIsKilled>,
) {
    for collision_event in event_collision_detected.read() {
        info!(
            "handle_health for actor: {:?}",
            collision_event.entity_a_actor_kind
        );

        //TODO check if other entity has health
        //TODO if so than apply damage
        //TODO check if dead and send event
    }
}

pub(crate) fn handle_event_actor_is_killed(
    mut event_actor_is_killed: EventReader<EventActorIsKilled>,
) {
    for event in event_actor_is_killed.read() {
        debug!("Actor is killed! : {:?}", event.actor_type);

        match event.actor_type {
            ComponentActorKind::PlayerKnight => {
                //TODO game over or something
            }
            ComponentActorKind::BigZombie => {
                //TODO update score -> create system for updating ui
                //TODO when actor is killed, destroy entity in world
            }
            ComponentActorKind::Wall => {
                //* ignore, walls cannot be destroyed */
            }
        }
    }
}

pub(crate) fn update_camera_position(
    player_query: Query<&Transform, (With<ComponentPlayerTag>, Without<ComponentCameraTag>)>,
    mut camera_query: Query<
        &mut Transform,
        (With<ComponentCameraTag>, Without<ComponentPlayerTag>),
    >,
) {
    let player_transform: &Transform = player_query.single();
    let mut camera_transform: Mut<Transform> = camera_query.single_mut();

    camera_transform.translation = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        0.0,
    );
}
