//TODO move to separate files
//TODO rename brain_2
//TODO add remark to guy about the trick used
//TODO remove brain

use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct ComponentAiBrain2 {}

impl ComponentAiBrain2 {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

pub(crate) trait AIAction {}

pub(crate) trait AIConsideration {}

//TODO add tests for logic

//TODO create some kind of builder for this
pub(crate) fn build_ai_set_stay_attacking_when_in_range() {}
