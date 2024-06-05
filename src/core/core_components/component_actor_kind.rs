use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug, PartialEq, Clone, Copy)]
#[reflect(Resource)]
pub(crate) enum ComponentActorKind {
    #[default]
    PlayerKnight, //* = player */
    BigZombie,
    Wall,
}
