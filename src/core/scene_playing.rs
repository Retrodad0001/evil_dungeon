use crate::retro_asset_management::prelude::*;
use bevy::prelude::*;

use crate::CameraTag;

pub(crate) fn load_assets(
    mut commands: Commands,
    bevy_asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = bevy_asset_server.load("sprites/atlas.png");

    let layout_dimensions = Vec2::new(1056.0, 528.0); //TODO get this form json

    let layout: TextureAtlasLayout = TextureAtlasLayout::new_empty(layout_dimensions);

    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation

    commands.spawn((SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: 1, //TODO this should be different and using the import tool
        },
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    },));
}
pub(crate) fn setup_camera(mut commands: Commands) {
    let mut camera_bundle: Camera2dBundle = Camera2dBundle::default();
    // change the settings we want to change:
    camera_bundle.projection.scale = 2.0;

    commands.spawn((camera_bundle, CameraTag::new()));
}
