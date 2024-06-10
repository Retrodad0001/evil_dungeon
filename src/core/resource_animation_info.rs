use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
    ComponentAnimationClip, ComponentAnimationClipKind, TexturePackerAtlasInfo, BIG_ZOMBIE_IDLE_0,
    BIG_ZOMBIE_IDLE_1, BIG_ZOMBIE_IDLE_2, BIG_ZOMBIE_IDLE_3, BIG_ZOMBIE_RUN_0, BIG_ZOMBIE_RUN_1,
    BIG_ZOMBIE_RUN_2, BIG_ZOMBIE_RUN_3, KNIGHT_IDLE_0, KNIGHT_IDLE_1, KNIGHT_IDLE_2, KNIGHT_IDLE_3,
    KNIGHT_RUN_0, KNIGHT_RUN_1, KNIGHT_RUN_2, KNIGHT_RUN_3,
};

#[derive(Resource, Debug, Default, Reflect)]
pub(crate) struct ResourceAnimationInfo {
    pub(crate) animation_clips: HashMap<ComponentAnimationClipKind, ComponentAnimationClip>,
}

impl ResourceAnimationInfo {
    pub(crate) fn new() -> Self {
        ResourceAnimationInfo {
            animation_clips: HashMap::new(),
        }
    }

    pub(crate) fn add_animation_clips(&mut self, resource_atlas_info: &TexturePackerAtlasInfo) {
        let animation_frames: Vec<i32> = vec![
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_0) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_1) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_2) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(KNIGHT_IDLE_3) as i32,
        ];

        self.animation_clips.insert(
            ComponentAnimationClipKind::KnightIdle,
            ComponentAnimationClip::new(
                ComponentAnimationClipKind::KnightIdle,
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
            ComponentAnimationClipKind::KnightMoving,
            ComponentAnimationClip::new(
                ComponentAnimationClipKind::KnightMoving,
                animation_frames,
                0.1,
                false,
            ),
        );

        let animation_frames: Vec<i32> = vec![
            resource_atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_IDLE_0) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_IDLE_1) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_IDLE_2) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_IDLE_3) as i32,
        ];

        self.animation_clips.insert(
            ComponentAnimationClipKind::BigZombieIdle,
            ComponentAnimationClip::new(
                ComponentAnimationClipKind::BigZombieIdle,
                animation_frames,
                0.2,
                false,
            ),
        );

        let animation_frames: Vec<i32> = vec![
            resource_atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_RUN_0) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_RUN_1) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_RUN_2) as i32,
            resource_atlas_info.get_bevy_atlas_index_by_file_name(BIG_ZOMBIE_RUN_3) as i32,
        ];

        self.animation_clips.insert(
            ComponentAnimationClipKind::BigZombieMoving,
            ComponentAnimationClip::new(
                ComponentAnimationClipKind::KnightMoving,
                animation_frames,
                0.1,
                false,
            ),
        );
    }
}
