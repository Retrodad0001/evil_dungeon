use bevy::{prelude::*, scene::ron::de};

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentHealth {
    pub(crate) current_health: i32,
}

impl ComponentHealth {
    pub(crate) fn new(current_health: i32) -> Self {
        debug_assert!(
            current_health > 0,
            "initial current_health must be greater than 0"
        );

        Self { current_health }
    }

    pub(crate) fn take_damage(&mut self, damage_amount: i32) {
        self.current_health -= damage_amount;
    }

    pub(crate) fn is_dead(&self) -> bool {
        self.current_health <= 0
    }
}

//TODO show health on screen and update this stuff
