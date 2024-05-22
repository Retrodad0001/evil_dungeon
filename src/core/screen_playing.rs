use crate::core::prelude::*;
use crate::tiw_asset_management::prelude::*;
use bevy::prelude::*;

use crate::ComponentCameraTag;

pub(crate) fn load_assets(
    bevy_asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_packer_info: ResMut<ResourceAtlasInfo>,
) {
    let atlas_dto: TexturePackerJsonDTO = create_dto_from_json_file();
    let texture_atlas_layout: TextureAtlasLayout =
        texture_packer_info.setup_bevy_spite_atlas(atlas_dto);
    let texture_atlas_layout_handle: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(texture_atlas_layout);
    texture_packer_info.texture_atlas_layout_handle = texture_atlas_layout_handle;

    let atlas_texture: Handle<Image> = bevy_asset_server.load("sprites/atlas.png");
    texture_packer_info.atlas_texture_handle = atlas_texture;
}

pub(crate) fn setup_camera(mut commands: Commands) {
    let mut camera_bundle: Camera2dBundle = Camera2dBundle::default();
    // change the settings we want to change:
    camera_bundle.projection.scale = 0.3;
    commands.spawn((camera_bundle, ComponentCameraTag::new()));
}

pub(crate) fn load_level(mut commands: Commands, atlas_info: ResMut<ResourceAtlasInfo>) {
    let index_knight_idle: usize = atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_0);

    commands.spawn(KnightBundle::new(
        atlas_info.atlas_texture_handle.clone(),
        atlas_info.texture_atlas_layout_handle.clone(),
        index_knight_idle,
        Vec2::new(50.0, 40.0),
    ));
}
