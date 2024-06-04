use crate::ResourceDebugSettings;
use bevy::prelude::*;
use iyes_perf_ui::PerfUiCompleteBundle;

use crate::core::prelude::*;

pub(crate) fn debug_show_pivot_points(
    debug_settings: Res<ResourceDebugSettings>,
    query_all_entities: Query<(&Name, &Transform), (Without<ComponentCameraTag>,)>,
    mut gizmos: Gizmos,
) {
    if debug_settings.show_debug_info {
        query_all_entities
            .into_iter()
            .for_each(|(_name, transform): (&Name, &Transform)| {
                let position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y);
                gizmos.circle_2d(position, 0.5, Color::LIME_GREEN);
            });
    }
}

pub(crate) fn debug_show_collision_bounds(
    debug_settings: Res<ResourceDebugSettings>,
    query_all_entities: Query<
        (&Name, &Transform, &ComponentCollision),
        (Without<ComponentCameraTag>,),
    >,
    mut gizmos: Gizmos,
) {
    if debug_settings.show_debug_info {
        query_all_entities.into_iter().for_each(
            |(_name, transform, component_collision): (&Name, &Transform, &ComponentCollision)| {
                let position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y)
                    + component_collision.offset;
                let size = Vec2::new(
                    component_collision.bounds_width,
                    component_collision.bounds_height,
                );
                const ROTATION: f32 = 0.;

                gizmos.rect_2d(position, ROTATION, size, Color::RED);
            },
        );
    }
}

pub(crate) fn debug_show_perf_stats(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}

pub(crate) fn enable_disable_debug_console_with_f12(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_settings: ResMut<ResourceDebugSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::F12) {
        debug_settings.show_debug_info = !debug_settings.show_debug_info;

        if debug_settings.show_debug_info {
            debug!("enable debug console");
        } else {
            debug!("disable debug console");
        }
    }
}
