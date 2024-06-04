mod event_actor_is_killed;
mod event_collision_detected;

pub(crate) mod prelude {
    pub(crate) use super::event_actor_is_killed::EventActorIsKilled;
    pub(crate) use super::event_collision_detected::EventCollisionDetected;
}
