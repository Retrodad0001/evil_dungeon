use bevy::{prelude::*, utils::hashbrown::HashMap};

use super::prelude::{AnimationClipKind, TiwAnimationClip};
use crate::tiw_asset_management::prelude::*;

#[derive(Resource, Debug, Default, Reflect)]
pub(crate) struct ResourceAnimationInfo {
    pub(crate) animation_clips: HashMap<AnimationClipKind, TiwAnimationClip>,
}

impl ResourceAnimationInfo {
    pub(crate) fn new() -> Self {
        ResourceAnimationInfo {
            animation_clips: HashMap::new(),
        }
    }

    pub(crate) fn add_animation_clips(&mut self, resource_atlas_info: &ResourceAtlasInfo) {
        let animation_frames: Vec<i32> = vec![
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_0) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_1) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_2) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_3) as i32,
        ];

        self.animation_clips.insert(
            AnimationClipKind::ClipKnightIdle,
            TiwAnimationClip::new(
                AnimationClipKind::ClipKnightIdle,
                animation_frames,
                0.2,
                false,
            ),
        );

        let animation_frames: Vec<i32> = vec![
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_RUN_0) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_RUN_1) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_RUN_2) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_RUN_3) as i32,
        ];

        self.animation_clips.insert(
            AnimationClipKind::ClipKnightMoving,
            TiwAnimationClip::new(
                AnimationClipKind::ClipKnightMoving,
                animation_frames,
                0.1,
                false,
            ),
        );
    }
}
