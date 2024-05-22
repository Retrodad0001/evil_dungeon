use crate::ResourceDebugSettings;
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts};
use iyes_perf_ui::PerfUiCompleteBundle;

pub(crate) fn debug_console(
    mut contexts: EguiContexts,
    mut debug_settings: ResMut<ResourceDebugSettings>,
    mut exit: EventWriter<AppExit>,

    query_all_entities_with_name: Query<(&Name, &Transform)>,
) {
    egui::Window::new("-DEBUG CONSOLE -").show(contexts.ctx_mut(), |ui| {
        ui.label(format!(
            "version {:?}",
            debug_settings.game_version_number.as_str()
        ));

        ui.checkbox(&mut debug_settings.show_pivot_points, "Show Pivot Points");

        ui.separator();
        let esc_pressed: egui::Response = ui.button("click here to exit ! ");
        if esc_pressed.clicked() {
            exit.send(AppExit);
        }
        ui.separator();

        ui.label("-- Entities --");
        query_all_entities_with_name
            .iter()
            .for_each(|(name, transform): (&Name, &Transform)| {
                let entity_text = format!(
                    "{:?} [ {:?} , {:?} ]",
                    name, transform.translation.x, transform.translation.y
                );

                ui.label(entity_text.as_str());
            });
    });
}

pub(crate) fn debug_show_pivot_points(
    debug_settings: Res<ResourceDebugSettings>,
    query_all_entities: Query<(&Name, &Transform)>,
    mut gizmos: Gizmos,
) {
    if debug_settings.show_pivot_points {
        query_all_entities
            .iter()
            .for_each(|(_name, transform): (&Name, &Transform)| {
                let position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y);
                gizmos.circle_2d(position, 2.0, Color::LIME_GREEN);
            });
    }
}

pub(crate) fn debug_show_perf_stats(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
    //TODO why no FPS is showed in the debug console
}
