use bevy::prelude::*;

#[derive(Component, Debug)]

pub(crate) struct ComponentCanMove {
    pub(crate) direction: Vec2, //TODO use bevy_math::Dir2; -->use Dir2   crates/bevy_math/src/direction.rs
    pub(crate) current_velocity: Vec2,
    max_speed: f32,
}

impl ComponentCanMove {
    pub(crate) fn new(max_speed: f32) -> Self {
        Self {
            max_speed,
            direction: Vec2::ZERO,
            current_velocity: Vec2::new(0.0, 0.0),
        }
    }

    #[inline(always)]
    pub(crate) fn calculate_velocity_no_slerp(&mut self, delta_time: &f32) {
        if self.direction.length() <= 0.0 {
            self.current_velocity = Vec2::new(0.0, 0.0);
            return;
        }

        self.current_velocity = self.direction * self.max_speed * *delta_time;

        debug_assert!(
            self.current_velocity.length() > 0.0,
            "current_velocity should not be zero at this point, it was ---> {:?}",
            self.current_velocity
        );
    }
}
