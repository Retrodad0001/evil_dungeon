use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Component, Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub(crate) struct ComponentCollision {
    pub(crate) offset: Vec2,
    pub(crate) bounds_width: f32,
    pub(crate) bounds_height: f32,
}

impl ComponentCollision {
    pub(crate) fn new(offset: Vec2, bounds_width: f32, bounds_height: f32) -> Self {
        Self {
            offset,
            bounds_width,
            bounds_height,
        }
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
        let offset: Vec2 = Vec2::new(0.0, 0.0);
        let component_collision: ComponentCollision = ComponentCollision::new(offset, 10.0, 10.0);
        let translation: Vec3 = Vec3::new(10.0, 20.0, 0.0);
        let aabb2d: Aabb2d = component_collision.get_aabb2d_bounds(&translation);

        assert_eq!(aabb2d.min, Vec2::new(10.0, 20.0));
        assert_eq!(aabb2d.max, Vec2::new(20.0, 30.0));
    }

    #[test]
    fn test_get_aabb2d_bounds_with_positive_offset() {
        let offset: Vec2 = Vec2::new(1.0, 1.0);
        let component_collision: ComponentCollision = ComponentCollision::new(offset, 10.0, 10.0);
        let translation: Vec3 = Vec3::new(10.0, 20.0, 0.0);
        let aabb2d: Aabb2d = component_collision.get_aabb2d_bounds(&translation);

        assert_eq!(aabb2d.min, Vec2::new(11.0, 21.0));
        assert_eq!(aabb2d.max, Vec2::new(21.0, 31.0));
    }

    #[test]
    fn test_get_aabb2d_bounds_with_negative_offset() {
        let offset: Vec2 = Vec2::new(-1.0, -1.0);
        let component_collision: ComponentCollision = ComponentCollision::new(offset, 10.0, 10.0);
        let translation: Vec3 = Vec3::new(10.0, 20.0, 0.0);
        let aabb2d: Aabb2d = component_collision.get_aabb2d_bounds(&translation);

        assert_eq!(aabb2d.min, Vec2::new(9.0, 19.0));
        assert_eq!(aabb2d.max, Vec2::new(19.0, 29.0));
    }
}
