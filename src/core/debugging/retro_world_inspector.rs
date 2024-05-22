use crate::{tiw_asset_management::prelude::*, ResourceDebugSettings};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub(crate) fn word_inspector(
    mut contexts: EguiContexts,
    debug_settings: Res<ResourceDebugSettings>,
) {
    let show_pivot_points: &bool = &debug_settings.show_pivot_points;

    egui::Window::new("INSPECTOR").show(contexts.ctx_mut(), |ui| {
        //    ui.checkbox(show_pivot_points, "pivot points");
        //  ui.button("ESC to exit");
    });

    //TODO exit button
    //TODO show
}

pub(crate) fn debug_show_pivot_points() {}
