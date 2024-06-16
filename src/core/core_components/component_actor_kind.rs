use bevy::prelude::*;

#[derive(Component, Default, Debug, PartialEq, Clone, Copy)]
pub(crate) enum ComponentActorKind {
    //* this is enough for teh sake of this example in bevy engine
    #[default]
    PlayerKnight, //* = player */
    BigZombie,
    Wall,
    Door,
    Axe,
    Wizard,
    WizardSpawn,
    SomeBoss,
}
