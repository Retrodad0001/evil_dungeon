use bevy::prelude::*;

use crate::core::prelude::*;

#[derive(Bundle)]
pub(crate) struct TiWCamera2dBundle {
    camera_tag: ComponentCameraTag,
    camera_2d_bundle: Camera2dBundle,
}

impl TiWCamera2dBundle {
    pub fn new() -> Self {
        let mut camera_bundle: Camera2dBundle = Camera2dBundle::default();
        // change the settings we want to change:
        camera_bundle.projection.scale = 0.3;

        Self {
            camera_tag: ComponentCameraTag::new(),
            camera_2d_bundle: camera_bundle,
        }
    }
}
