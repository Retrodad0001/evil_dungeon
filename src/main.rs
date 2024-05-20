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
    let mut app: App = App::new();

    add_plugins(&mut app);
    add_resources(&mut app);
    add_events(&mut app);

    add_screen_loading_systems(&mut app);

    add_screen_menu_systems(&mut app);

    add_screen_playing_on_enter_systems(&mut app);
    add_screen_playing_systems(&mut app);
    add_screen_playing_on_exit_systems(&mut app);
    add_screen_playing_debug_systems(&mut app);

    app.insert_state(ScreenState::Playing);

    app.run();
}

fn add_plugins(app: &mut App) {
    app.add_plugins(
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
    );

    app.add_plugins(EguiPlugin);
}

fn add_resources(app: &mut App) {
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    app.insert_resource(DebugSettings::new());
    app.insert_resource(GameInfo::new());
    app.insert_resource(TexturePackerInfo::new());
}

fn add_events(app: &mut App) {
    app.add_event::<EventPlayerIsHit>();
}

fn add_screen_loading_systems(_app: &mut App) {}

fn add_screen_menu_systems(_app: &mut App) {}

fn add_screen_playing_on_enter_systems(app: &mut App) {
    app.add_systems(
        OnEnter(ScreenState::Playing),
        (load_assets, setup_camera).chain(),
    );
}

fn add_screen_playing_systems(app: &mut App) {}

fn add_screen_playing_on_exit_systems(app: &mut App) {}

fn add_screen_playing_debug_systems(app: &mut App) {
    app.add_systems(
        Update,
        word_inspector.run_if(in_state(ScreenState::Playing)),
    );
}
