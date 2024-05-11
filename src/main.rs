use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_egui::EguiPlugin;
use core::prelude::*;
use retro_asset_management::prelude::*;

mod core;
mod retro_ai;
mod retro_animation;
mod retro_asset_management;
mod retro_physics;
mod retro_tilemap;

fn main() {
    App::new() //TODO pixel perferct scaling
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "EVIL DUNGEON !".into(),
                        name: Some("MyWindow".into()),
                        resolution: (1920., 1080.).into(),
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        visible: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(EguiPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(DebugSettings::new())
        .insert_resource(GameInfo::new())
        .insert_resource(TexturePackerInfo {})
        .insert_state(SceneState::Playing)
        .add_systems(
            Startup,
            (
                load_assets,
                setup_camera.run_if(in_state(SceneState::Playing)),
            )
                .chain(),
        )
        //TODO add debug stuff in plugin so it can be toggled on and off
        .add_systems(
            Update,
            ui_example_system.run_if(in_state(SceneState::Playing)),
        )
        .run();
}
