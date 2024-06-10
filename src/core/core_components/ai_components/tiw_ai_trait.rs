use bevy::prelude::*;

#[bevy_trait_query::queryable]
pub(crate) trait TiWAI {
    fn process(&self);
}
