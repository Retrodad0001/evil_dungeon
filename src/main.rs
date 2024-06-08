use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_light_2d::plugin::Light2dPlugin;

use core::prelude::*;
use iyes_perf_ui::PerfUiPlugin;
use tiw_asset_management::prelude::*;

mod core;
mod tiw_asset_management;
mod tiw_tilemap;

fn main() {
    let mut app: App = App::new();

    add_plugins(&mut app);
    add_resources(&mut app);
    add_type_registrations(&mut app);
    add_events(&mut app);

    add_screen_loading_systems(&mut app);

    add_screen_menu_systems(&mut app);

    add_screen_playing_on_enter_systems(&mut app);
    add_screen_playing_systems(&mut app);

    app.insert_state(ScreenState::Playing);

    #[cfg(debug_assertions)]
    add_screen_playing_debug_systems(&mut app);

    app.run();
}

fn add_plugins(app: &mut App) {
    let wgpu_setting: WgpuSettings = WgpuSettings {
        backends: Some(Backends::VULKAN),
        ..Default::default()
    };

    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,evil_dungeon=debug".into(),
                level: bevy::log::Level::DEBUG,
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            // setup vulcan backend suppress warnings for now (zie issue #5 or upstream bevy issue #9975)
            //, remove this when bug is fixed
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(wgpu_setting),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "EVIL DUNGEON BEVY ENGINE 2D Demo!".into(),
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

    app.add_plugins(Light2dPlugin {});
}

fn add_type_registrations(app: &mut App) {
    app.register_type::<ComponentCanMove>();
    app.register_type::<ComponentCanAnimate>();
    app.register_type::<ComponentActorKind>();
    app.register_type::<ComponentCanCollide>();
    app.register_type::<ComponentHasHealth>();
    app.register_type::<TexturePackerAtlasInfo>();
    app.register_type::<ResourceAnimationInfo>();
    app.register_type::<ComponentPlayerTag>();
}

fn add_resources(app: &mut App) {
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    app.insert_resource(ResourceDebugSettings::new());
    app.insert_resource(ResourceGeneralGameState::new());
    app.insert_resource(ResourceGameSettings::new());
    app.insert_resource(TexturePackerAtlasInfo::new());
    app.insert_resource(ResourceAnimationInfo::new());
}

fn add_events(app: &mut App) {
    app.add_event::<EventCollisionDetected>();
    app.add_event::<EventActorIsKilled>();
}

fn add_screen_loading_systems(_app: &mut App) {}

fn add_screen_menu_systems(_app: &mut App) {}

fn add_screen_playing_on_enter_systems(app: &mut App) {
    app.add_systems(
        OnEnter(ScreenState::Playing),
        (load_assets, setup_animations, setup_camera, new_level).chain(),
    );
}

fn add_screen_playing_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            do_fancy_ai_for_enemies,
            calculate_direction_for_player,
            calculate_direction_for_enemies,
            animate_all,
            update_camera_position,
            handle_health_when_event_collision_for_all,
            handle_event_actor_is_killed,
        )
            .run_if(in_state(ScreenState::Playing)),
    );

    app.add_systems(
        FixedUpdate,
        (
            physics_determine_actor_collision_for_all,
            calculate_velocity_for_player,
            calculate_velocity_for_enemies,
        )
            .chain()
            .run_if(in_state(ScreenState::Playing)),
    );
}

fn add_screen_playing_debug_systems(app: &mut App) {
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(PerfUiPlugin);
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
    app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);
    app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);

    app.add_systems(
        Startup,
        (debug_show_perf_stats,).run_if(in_state(ScreenState::Playing)),
    );

    app.add_systems(
        Update,
        (
            debug_show_pivot_points,
            debug_show_collision_bounds,
            debug_show_grid_coordinates,
            enable_disable_debug_console_with_f12,
        )
            .run_if(in_state(ScreenState::Playing)),
    );
}
