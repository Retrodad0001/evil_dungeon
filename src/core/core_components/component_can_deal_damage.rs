use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentCanDealDamage {
    pub(crate) damage_amount: i32,
}

impl ComponentCanDealDamage {
    pub(crate) fn new(damage_amount: i32) -> Self {
        Self { damage_amount }
    }
}
