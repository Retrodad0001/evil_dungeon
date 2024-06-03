use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentCollision {
    pub(crate) colliding_entities: Vec<Entity>, //TODO change to custom object with more info than entity
}

impl ComponentCollision {
    pub(crate) fn new() -> Self {
        Self {
            colliding_entities: Vec::new(),
        }
    }

    pub(crate) fn get_aabb2d_bounds(&self, transform: &Transform) -> Aabb2d {
        Aabb2d {
            min: Vec2::new(transform.translation.x, transform.translation.y),
            max: Vec2::new(
                transform.translation.x + 16.0, //TODO const
                transform.translation.y + 16.0, //TODO const
            ),
        }
    }
}
