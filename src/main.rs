use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowTheme},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use components::prelude::*;

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "EVIL DUNGEON !".into(),
                name: Some("MyWindow".into()),
                resolution: (800., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                visible: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(Update, ui_example_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::BLUE),
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        CameraTag::new(),
    ));
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("INSPECTOR").show(contexts.ctx_mut(), |ui| {
        ui.label("Test stuff");
    });
}
