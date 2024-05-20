use bevy::prelude::*;

#[derive(Bundle)]
pub(crate) struct WizardBundle {}

impl WizardBundle {
    pub fn new() -> Self {
        Self {}
    }
}
