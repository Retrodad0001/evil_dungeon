use bevy::prelude::*;
//TODO use bevy_math::Dir2; -->use Dir2   crates/bevy_math/src/direction.rs

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentCanMove {
    pub(crate) direction: Vec2,
    pub(crate) current_velocity: Vec2,
    max_speed: f32,
}

impl ComponentCanMove {
    pub(crate) fn new() -> Self {
        Self {
            max_speed: 80.0,
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

        self.current_velocity = self.direction * self.max_speed * *delta_time;

        debug_assert!(
            self.current_velocity.length() > 0.0,
            "current_velocity should not be zero at this point, it was ---> {:?}",
            self.current_velocity
        );
    }
}
