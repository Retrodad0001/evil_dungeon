use bevy::math::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentMovement {
    pub(crate) direction: Vec2,
    pub(crate) current_velocity: Vec2,
    speed: f32,
}

impl ComponentMovement {
    pub(crate) fn new() -> Self {
        Self {
            speed: 80.0, //TODO rename to max_speed and add param in new
            direction: Vec2::new(0.0, 0.0),
            current_velocity: Vec2::new(0.0, 0.0),
        }
    }

    #[inline(always)]
    pub(crate) fn calculate_velocity(&mut self, delta_time: &f32) {
        if self.direction.length() <= 0.0 {
            self.current_velocity = Vec2::new(0.0, 0.0);
            return;
        }

        self.current_velocity = self.direction * self.speed * *delta_time;

        debug_assert!(
            self.current_velocity.length() > 0.0,
            "current_velocity should not be zero at this point, it was ---> {:?}",
            self.current_velocity
        );
    }
}
