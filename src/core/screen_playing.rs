use crate::core::prelude::*;
use crate::tiw_animation::prelude::*;
use crate::tiw_asset_management::prelude::*;
use crate::tiw_camera::prelude::*;
use bevy::prelude::*;

pub(crate) fn load_assets(
    bevy_asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_packer_info: ResMut<ResourceAtlasInfo>,
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
    resource_atlas_info: ResMut<ResourceAtlasInfo>,
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

pub(crate) fn load_level(mut commands: Commands, atlas_info: ResMut<ResourceAtlasInfo>) {
    debug!("start - load_level");
    let index_knight_idle: usize = atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_0);

    commands.spawn(KnightBundle::new(
        atlas_info.atlas_texture_handle.clone(),
        atlas_info.texture_atlas_layout_handle.clone(),
        index_knight_idle,
        Vec2::new(50.0, 40.0),
    ));
    debug!("end - load_level");
}

pub(crate) fn calculate_direction_for_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut ComponentMovement, &ComponentHandleInput)>,
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

    let mut player: (Mut<ComponentMovement>, &ComponentHandleInput) = player_query.single_mut();

    player.0.direction = direction.normalize_or_zero();
}

pub(crate) fn calculate_ai_next_task_for_enemies() {
    //TODO calculate_ai_next_task_for_enemies
}

pub(crate) fn calculate_direction_for_enemies() {
    //TODO calculate_direction_for_enemies
}

pub(crate) fn animate_all(
    mut animation_entities_query: Query<(
        &ActorKind,
        &ComponentMovement,
        &mut ComponentAnimation,
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
        let atlas_index: i32 = animation.determine_current_atlas_index_for_animation(
            actor_kind,
            &movement.direction,
            delta_time,
            &animation_info,
        );

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

pub(crate) fn physics_determine_collision_for_all() {
    //TODO physics_determine_collision_for_all
}

pub(crate) fn update_camera_position() {
    //TODO update_camera_position and more stuff
}
