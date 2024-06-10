use super::events::prelude::EventCollisionDetected;
use super::resource_general_game_state;
use crate::core::prelude::*;
use crate::tiw_asset_management::prelude::*;
use crate::tiw_tilemap::prelude::MapGenerationInput;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

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
    let map_generation_input: MapGenerationInput = MapGenerationInput::new(10, 10);

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
    let spawn_location: Vec2 = resource_game_state
        .tiw_tile_map
        .get_world_position_from_tile_position(1, 1);

    let index_knight_idle: usize = atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_0);
    commands.spawn(KnightBundle::new(
        atlas_info.atlas_texture_handle.clone(),
        atlas_info.texture_atlas_layout_handle.clone(),
        index_knight_idle,
        spawn_location,
        100,
    ));

    //spawn enemies in level
    let spawn_location: Vec2 = resource_game_state
        .tiw_tile_map
        .get_world_position_from_tile_position(5, 5);

    let index_big_zombie_idle: usize =
        atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_IDLE_0);
    commands.spawn(BigZombieBundle::new(
        atlas_info.atlas_texture_handle.clone(),
        atlas_info.texture_atlas_layout_handle.clone(),
        index_big_zombie_idle,
        spawn_location,
        50,
    ));

    debug!("end - new_level");
}

pub(crate) fn calculate_direction_for_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut ComponentCanMove, &ComponentPlayerTag, &mut Transform)>,
) {
    let mut direction_vector: Vec2 = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_vector.x = -1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_vector.x = 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction_vector.y = 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_vector.y = -1.0;
    }

    let mut player: (Mut<ComponentCanMove>, &ComponentPlayerTag, Mut<Transform>) =
        player_query.single_mut();

    direction_vector = direction_vector.normalize_or_zero();
    player.0.direction = direction_vector;
}

pub(crate) fn do_fancy_ai_for_enemies() {}

pub(crate) fn calculate_direction_for_enemies() {}

pub(crate) fn animate_all(
    mut animation_entities_query: Query<(
        &ComponentActorKind,
        &ComponentCanMove,
        &mut ComponentCanAnimate,
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

pub(crate) fn calculate_velocity_for_player(
    mut movement_player: Query<(&mut Transform, &mut ComponentCanMove), With<ComponentPlayerTag>>,
    resource_game_state: Res<resource_general_game_state::ResourceGeneralGameState>,
    time: Res<Time>,
) {
    let delta_time: f32 = time.delta_seconds();

    let mut player: (Mut<Transform>, Mut<ComponentCanMove>) = movement_player.single_mut();

    player.1.calculate_velocity_no_slerp(&delta_time);

    let direction: Vec2 = player.1.direction;
    let mut ray_length: f32 = 10.0;

    if direction.y < 0.0 {
        ray_length -= 12.0;
    } //* in future use real raycast with builtin bevy bounds functionality

    let additional_distance: Vec2 = direction * ray_length;

    let search_tile_location: Vec2 =
        Vec2::new(player.0.translation.x, player.0.translation.y) + additional_distance;

    let is_blocking_tile_in_that_direction: bool = resource_game_state
        .tiw_tile_map
        .is_blocking_tile_at_location(search_tile_location.x, search_tile_location.y);

    if is_blocking_tile_in_that_direction {
        info!("player is blocked by wall");
    } else {
        let new_location = player.0.translation
            + Vec3::new(
                player.1.current_velocity.x,
                player.1.current_velocity.y,
                0.0,
            );
        player.0.translation = new_location;
    }
}

pub(crate) fn calculate_velocity_for_enemies(
    mut movement_entities_query: Query<
        (&mut Transform, &mut ComponentCanMove),
        Without<ComponentPlayerTag>,
    >,
    time: Res<Time>,
) {
    let delta_time: f32 = time.delta_seconds();

    for (mut transform, mut movement) in movement_entities_query.iter_mut() {
        movement.calculate_velocity_no_slerp(&delta_time);
        transform.translation += Vec3::new(
            movement.current_velocity.x,
            movement.current_velocity.y,
            0.0,
        );
    }
}

pub(crate) fn physics_determine_actor_collision_for_all(
    collision_entities_query: Query<(
        Entity,
        &Transform,
        &ComponentCanCollide,
        &ComponentActorKind,
    )>,
    mut event_collision_detected: EventWriter<EventCollisionDetected>,
) {
    //TODO: optimize this : only trigger event when collision is detected once OnEnter or OnLeave

    for (entity_a, transform_a, collision_a, actor_a_kind) in collision_entities_query.iter() {
        for (entity_b, transform_b, collision_b, actor_b_kind) in collision_entities_query.iter() {
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
                    *actor_a_kind,
                    *actor_b_kind,
                    entity_a,
                    entity_b,
                ));

                //* debug!("collision between {:?} and {:?}", name_a, name_b);
            }
        }
    }
}

pub(crate) fn collision_event_handle_damage_dealing_and_health_for_all(
    mut event_collision_detected: EventReader<EventCollisionDetected>,
    mut event_actor_is_killed: EventWriter<EventActorIsKilled>,
    query_actor_a: Query<&ComponentCanDealDamage>,
    mut query_actor_b: Query<&mut ComponentHasHealth>,
) {
    for collision_event in event_collision_detected.read() {
        let entity_a: Entity = collision_event.entity_a;
        let entity_b: Entity = collision_event.entity_b;

        //*https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Query.html#method.get_many_mut *//
        let vec_entity_a: [Entity; 1] = [entity_a];
        let [entity_a_damage_dealing]: [&ComponentCanDealDamage; 1] =
            query_actor_a.many(vec_entity_a);

        let vec_entity_b: [Entity; 1] = [entity_b];
        let [mut entity_b_health]: [Mut<ComponentHasHealth>; 1] =
            query_actor_b.many_mut(vec_entity_b);

        //BUG:check when entity's don't have health or damage dealing

        entity_b_health.current_health -= entity_a_damage_dealing.damage_amount;
        info!(
            "{:?} current_health: {:?}",
            collision_event.entity_b_actor_kind, entity_b_health.current_health
        );

        if entity_b_health.current_health <= 0 {
            //*is already marked dead so, it cannot go dead again */
            if entity_b_health.marked_as_dead {
                continue;
            }

            entity_b_health.marked_as_dead = true;

            event_actor_is_killed.send(EventActorIsKilled::new(
                collision_event.entity_b_actor_kind,
                entity_b,
            ));
        }
    }
}

pub(crate) fn actor_is_killed_event_handle_for_all(
    mut event_actor_is_killed: EventReader<EventActorIsKilled>,
) {
    for event in event_actor_is_killed.read() {
        info!("actor {:?} is killed", event.actor_kind);
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
