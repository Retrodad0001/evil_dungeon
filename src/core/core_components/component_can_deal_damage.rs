use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentCanDealDamage {
    pub(crate) current_damage_to_deal: u32,
}

impl ComponentCanDealDamage {
    pub(crate) fn new(current_damage_to_deal: u32) -> Self {
        Self {
            current_damage_to_deal,
        }
    }
}
