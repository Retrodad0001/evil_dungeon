mod component_actor_kind;
mod component_animation_clip;
mod component_animation_clip_kind;
mod component_animator;
mod component_camera_tag;
mod component_collision;
mod component_health;
mod component_movement;
mod component_player_tag;

pub(crate) mod prelude {

    pub(crate) use super::component_actor_kind::*;
    pub(crate) use super::component_animation_clip::*;
    pub(crate) use super::component_animation_clip_kind::*;
    pub(crate) use super::component_animator::*;
    pub(crate) use super::component_camera_tag::*;
    pub(crate) use super::component_collision::*;
    pub(crate) use super::component_health::*;
    pub(crate) use super::component_movement::*;
    pub(crate) use super::component_player_tag::*;
}
