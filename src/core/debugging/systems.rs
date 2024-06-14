use crate::ResourceDebugSettings;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use iyes_perf_ui::PerfUiCompleteBundle;

use crate::core::prelude::*;

pub(crate) fn draw_debug_console(
    debug_settings: Res<ResourceDebugSettings>,
    mut contexts: EguiContexts,
    query_all_entities: Query<(&Name, &Transform, &ComponentAI), (Without<ComponentTileType>,)>,
) {
    if !debug_settings.show_debug_info {
        return;
    }

    egui::Window::new("-- DEBUG CONSOLE --").show(contexts.ctx_mut(), |ui| {
        for (name, transform, ai) in query_all_entities.iter() {
            //format transform.translation.x, transform.translation.y with fixed decimal places

            ui.label(format!(
                "{:?}  [{:3.3},{:3.3}] - State: {:?}",
                name, transform.translation.x, transform.translation.y, ai.current_state
            ));
        }
    });
}

pub(crate) fn debug_show_pivot_points(
    debug_settings: Res<ResourceDebugSettings>,
    query_all_entities: Query<(&Name, &Transform), (Without<ComponentCameraTag>,)>,
    mut gizmos: Gizmos,
) {
    if !debug_settings.show_debug_info {
        return;
    }

    query_all_entities
        .into_iter()
        .for_each(|(_name, transform): (&Name, &Transform)| {
            let position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y);
            gizmos.circle_2d(position, 0.5, Color::LIME_GREEN);
        });
}

pub(crate) fn debug_show_collision_bounds(
    debug_settings: Res<ResourceDebugSettings>,
    query_all_entities: Query<
        (&Name, &Transform, &ComponentCanCollide),
        (Without<ComponentCameraTag>,),
    >,
    mut gizmos: Gizmos,
) {
    if !debug_settings.show_debug_info {
        return;
    }
    query_all_entities.into_iter().for_each(
        |(_name, transform, component_collision): (&Name, &Transform, &ComponentCanCollide)| {
            let position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y)
                + component_collision.offset;
            let size = Vec2::new(
                component_collision.bounds_width,
                component_collision.bounds_height,
            );
            const ROTATION: f32 = 0.;
            gizmos.rect_2d(position, ROTATION, size, Color::LIME_GREEN);
        },
    );
}

pub(crate) fn debug_draw_ai_stuff(
    debug_settings: Res<ResourceDebugSettings>,
    query_all_entities: Query<(&Transform, &ComponentAI), (Without<ComponentCameraTag>,)>,
    mut gizmos: Gizmos,
) {
    if !debug_settings.show_debug_info {
        return;
    }
    query_all_entities
        .into_iter()
        .for_each(|(transform, ai): (&Transform, &ComponentAI)| {
            let position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y);

            //* draw chase radius

            //* if there is an target and chasing draw chase line
            match ai.current_state {
                AiState::Chasing => {
                    gizmos.circle_2d(position, ai.chase_attack_range, Color::YELLOW);

                    if let Some(target) = ai.next_target_position {
                        let target_position = Vec2::new(target.x, target.y);
                        gizmos.line_2d(position, target_position, Color::YELLOW);
                    }
                }
                AiState::Idle => {}
                AiState::Wandering => {
                    gizmos.circle_2d(position, ai.chase_attack_range, Color::GREEN);

                    if let Some(target) = ai.next_target_position {
                        let target_position = Vec2::new(target.x, target.y);
                        gizmos.line_2d(position, target_position, Color::GREEN);
                    }
                }
                AiState::AttackMelee => {
                    gizmos.circle_2d(position, ai.chase_attack_range, Color::RED);

                    if let Some(target) = ai.next_target_position {
                        let target_position = Vec2::new(target.x, target.y);
                        gizmos.line_2d(position, target_position, Color::RED);
                    }
                }
                AiState::AttackingWithSpawningEnemies => todo!(),
                AiState::Fleeing => {}
            }
        });
}

pub(crate) fn debug_show_grid_coordinates(
    debug_settings: Res<ResourceDebugSettings>,
    query_all_entities: Query<
        (&Name, &Transform, &ComponentCanCollide, &ComponentActorKind),
        (Without<ComponentCameraTag>,),
    >,
    mut _gizmos: Gizmos,
) {
    if !debug_settings.show_debug_info {
        return;
    }
    query_all_entities.into_iter().for_each(
        |(_name, _transform, _component_collision, _actor_kind): (
            &Name,
            &Transform,
            &ComponentCanCollide,
            &ComponentActorKind,
        )| {
            // let position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y)
            //     + component_collision.offset;
            // let size = Vec2::new(
            //     component_collision.bounds_width,
            //     component_collision.bounds_height,
            // );
            // const ROTATION: f32 = 0.;

            // gizmos.rect_2d(position, ROTATION, size, Color::LIME_GREEN);

            //TODO debug draw text gizmos grid coordinates
        },
    );
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
