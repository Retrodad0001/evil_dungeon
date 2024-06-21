use bevy::prelude::*;

#[inline(always)]
pub(crate) fn is_in_range(actor_from: &Vec2, actor_to: &Vec2, range: f32) -> bool {
    let result: bool = get_distance_to_actor(actor_from, actor_to) <= range;
    result
}

#[inline(always)]
fn get_distance_to_actor(actor_from: &Vec2, actor_to: &Vec2) -> f32 {
    let result: f32 = Vec2::distance(*actor_from, *actor_to);
    result
}
