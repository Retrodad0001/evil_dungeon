use bevy::prelude::*;

#[derive(Component, Debug)]

pub(crate) struct ComponentPlayerTag;

impl ComponentPlayerTag {
    pub(crate) fn new() -> Self {
        Self
    }
}
