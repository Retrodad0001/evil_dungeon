use crate::core::prelude::*;

use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentAnimator {
    previous_animation_kind: ComponentAnimationClipKind,
    animation_frame_timer_sec: f32,
    current_animation_frame: i32,
}

impl ComponentAnimator {
    pub(crate) fn new(previous_animation_kind: ComponentAnimationClipKind) -> Self {
        Self {
            previous_animation_kind,
            animation_frame_timer_sec: 0.0,
            current_animation_frame: 0,
        }
    }

    #[inline(always)]
    pub(crate) fn determine_current_atlas_index_for_animation(
        &mut self,
        actor_kind: &ComponentActorKind,
        direction: &Vec2,
        delta_time: f32,
        animation_info: &Res<ResourceAnimationInfo>,
    ) -> Option<i32> {
        //determine the current animation clip based on the direction

        let animation_kind_option: Option<ComponentAnimationClipKind> =
            self.determine_animation_kind(actor_kind, direction);

        animation_kind_option.as_ref()?;

        let new_animation_kind: ComponentAnimationClipKind = animation_kind_option.unwrap();

        if let Some(new_animation_clip) = animation_info.animation_clips.get(&new_animation_kind) {
            if self.previous_animation_kind != new_animation_kind {
                self.current_animation_frame = 0;
                self.animation_frame_timer_sec = 0.0;
            }

            if self.animation_frame_timer_sec >= new_animation_clip.time_between_frames_sec {
                self.animation_frame_timer_sec = 0.0;

                if new_animation_clip.animation_frames.len() as i32
                    <= self.current_animation_frame + 1
                {
                    self.current_animation_frame = 0;
                } else {
                    self.current_animation_frame += 1;
                }
            }

            self.previous_animation_kind = new_animation_kind;
            self.animation_frame_timer_sec += delta_time;

            let current_atlas_index: i32 =
                new_animation_clip.animation_frames[self.current_animation_frame as usize];

            Some(current_atlas_index)
        } else {
            error!(
                "AnimationClipKind not found in determine_current_atlas_index_for_animation : {:?}",
                new_animation_kind
            );
            None
        }
    }

    #[inline(always)]
    fn determine_animation_kind(
        &mut self,
        actor_kind: &ComponentActorKind,
        direction: &Vec2,
    ) -> Option<ComponentAnimationClipKind> {
        let actor_is_moving: bool = direction.length() > 0.0;

        match actor_kind {
            ComponentActorKind::Knight => {
                if actor_is_moving {
                    Some(ComponentAnimationClipKind::ClipKnightMoving)
                } else {
                    Some(ComponentAnimationClipKind::ClipKnightIdle)
                }
            }
            ComponentActorKind::Wall => {
                //there is no animation for walls
                None
            }
        }
    }
}
