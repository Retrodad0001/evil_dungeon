use bevy::prelude::*;

#[derive(Component, Default, Debug, PartialEq, Clone, Copy)]
pub(crate) enum ComponentAiAction {
    #[default]
    Idle,
    Wandering,   //TODO and wandering time to this state (another struct)
    Chasing,     //TODO and chasing vars time to this state (another struct)
    AttackMelee, //TODO and AttackMelee vars time to this state (another struct)
}
