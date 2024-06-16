use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Component, Default)]

pub(crate) enum ComponentAnimationClipKind {
    #[default]
    KnightIdle = 0,
    KnightMoving = 1,
    BigZombieIdle = 2,
    BigZombieMoving = 3,
    DoorOpening = 4,
    DoorClose = 5,
    Axe = 6,
    WizardIdle = 7,
    WizardMoving = 8,
    WizardSpawnIdle = 9,
    WizardSpawnMoving = 10,
}
