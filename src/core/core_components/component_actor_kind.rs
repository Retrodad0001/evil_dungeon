use bevy::prelude::*;

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) enum ComponentActorKind {
    #[default]
    Knight,
    Wall,
}