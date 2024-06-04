use bevy::prelude::*;
use bevy_light_2d::light::AmbientLight2d;

use crate::core::prelude::*;

#[derive(Bundle)]
pub(crate) struct TiWCamera2dBundle {
    camera_tag: ComponentCameraTag,
    camera_2d_bundle: Camera2dBundle,
    ambient_light: AmbientLight2d,
}

impl TiWCamera2dBundle {
    pub fn new() -> Self {
        let mut camera_bundle: Camera2dBundle = Camera2dBundle::default();
        // change the settings we want to change:
        camera_bundle.projection.scale = 0.3;

        Self {
            camera_tag: ComponentCameraTag::new(),
            camera_2d_bundle: camera_bundle,
            ambient_light: AmbientLight2d {
                color: Color::WHITE,
                brightness: 0.1,
            },
        }
    }
}
