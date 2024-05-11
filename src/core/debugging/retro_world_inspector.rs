use bevy_egui::{egui, EguiContexts};

pub(crate) fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("INSPECTOR").show(contexts.ctx_mut(), |ui| {
        ui.label("Test stuff");
    });
}
