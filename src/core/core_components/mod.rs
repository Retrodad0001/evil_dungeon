mod component_actor_kind;
mod component_animation_clip;
mod component_animation_clip_kind;
mod component_camera_tag;
mod component_can_animate;
mod component_can_collide;
mod component_can_deal_damage;
mod component_can_move;
mod component_has_health;
mod component_player_tag;

pub(crate) mod prelude {

    pub(crate) use super::component_actor_kind::*;
    pub(crate) use super::component_animation_clip::*;
    pub(crate) use super::component_animation_clip_kind::*;
    pub(crate) use super::component_camera_tag::*;
    pub(crate) use super::component_can_animate::*;
    pub(crate) use super::component_can_collide::*;
    pub(crate) use super::component_can_deal_damage::*;
    pub(crate) use super::component_can_move::*;
    pub(crate) use super::component_has_health::*;
    pub(crate) use super::component_player_tag::*;
}
