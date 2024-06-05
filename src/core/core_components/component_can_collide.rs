use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::ComponentActorKind;

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentCanCollide {
    pub(crate) offset: Vec2,
    pub(crate) bounds_width: f32,
    pub(crate) bounds_height: f32,
    pub(crate) actor_kind: ComponentActorKind,
    pub(crate) collision_mask: Vec<ComponentActorKind>,
}

impl ComponentCanCollide {
    pub(crate) fn new(
        offset: Vec2,
        bounds_width: f32,
        bounds_height: f32,
        actor_kind: ComponentActorKind,
        collision_mask: Vec<ComponentActorKind>,
    ) -> Self {
        Self {
            offset,
            bounds_width,
            bounds_height,
            actor_kind,
            collision_mask,
        }
    }

    pub(crate) fn should_ignore_collision_processing(
        &self,
        entity_a: Entity,
        entity_b: Entity,
        collision_a: &ComponentCanCollide,
        collision_b: &ComponentCanCollide,
    ) -> bool {
       

        if collision_a.collision_mask.is_empty() {
            return true;
        }

        if !collision_a.collision_mask.contains(&collision_b.actor_kind) {
            return true;
        }

        //* make sure that the entity is not checking itself */
        if entity_a.to_bits() == entity_b.to_bits() {
            return true;
        }

        false
    }

    pub(crate) fn get_aabb2d_bounds(&self, translation: &Vec3) -> Aabb2d {
        Aabb2d {
            min: Vec2::new(translation.x, translation.y) + self.offset,
            max: Vec2::new(
                translation.x + self.bounds_width,
                translation.y + self.bounds_height,
            ) + self.offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_aabb2d_bounds_with_no_offset() {
        let collision_mask: Vec<ComponentActorKind> = Vec::new();
        let offset: Vec2 = Vec2::new(0.0, 0.0);
        let component_collision: ComponentCanCollide = ComponentCanCollide::new(
            offset,
            10.0,
            10.0,
            ComponentActorKind::PlayerKnight,
            collision_mask,
        );
        let translation: Vec3 = Vec3::new(10.0, 20.0, 0.0);
        let aabb2d: Aabb2d = component_collision.get_aabb2d_bounds(&translation);

        assert_eq!(aabb2d.min, Vec2::new(10.0, 20.0));
        assert_eq!(aabb2d.max, Vec2::new(20.0, 30.0));
    }

    #[test]
    fn test_get_aabb2d_bounds_with_positive_offset() {
        let collision_mask: Vec<ComponentActorKind> = Vec::new();
        let offset: Vec2 = Vec2::new(1.0, 1.0);
        let component_collision: ComponentCanCollide = ComponentCanCollide::new(
            offset,
            10.0,
            10.0,
            ComponentActorKind::PlayerKnight,
            collision_mask,
        );
        let translation: Vec3 = Vec3::new(10.0, 20.0, 0.0);
        let aabb2d: Aabb2d = component_collision.get_aabb2d_bounds(&translation);

        assert_eq!(aabb2d.min, Vec2::new(11.0, 21.0));
        assert_eq!(aabb2d.max, Vec2::new(21.0, 31.0));
    }

    #[test]
    fn test_get_aabb2d_bounds_with_negative_offset() {
        let collision_mask: Vec<ComponentActorKind> = Vec::new();
        let offset: Vec2 = Vec2::new(-1.0, -1.0);
        let component_collision: ComponentCanCollide = ComponentCanCollide::new(
            offset,
            10.0,
            10.0,
            ComponentActorKind::PlayerKnight,
            collision_mask,
        );
        let translation: Vec3 = Vec3::new(10.0, 20.0, 0.0);
        let aabb2d: Aabb2d = component_collision.get_aabb2d_bounds(&translation);

        assert_eq!(aabb2d.min, Vec2::new(9.0, 19.0));
        assert_eq!(aabb2d.max, Vec2::new(19.0, 29.0));
    }
}
