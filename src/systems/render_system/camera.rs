use core::f32::consts::PI;
use nalgebra::Isometry2;
use nalgebra::Point2;
use nalgebra::Translation2;
use nalgebra::Vector2;

const CAM_CENTER: Vector2<f32> = Vector2::new(400.0, 300.0);

pub struct Camera {
    pub transform: Isometry2<f32>,
    pub transform_next: Isometry2<f32>,
}

impl Camera {
    pub fn new(transform: Isometry2<f32>) -> Self {
        Camera {
            transform,
            transform_next: transform,
        }
    }

    pub fn modify(&self, vec: Vector2<f32>) -> Point2<f32> {
        let p = Point2::new(vec.x, vec.y);
        self.transform.inverse().transform_point(&p)
    }

    pub fn update(&mut self) {
        let diff = self.transform_next.translation.vector - self.transform.translation.vector;
        let diff = diff / 6.0;
        self.transform.translation = Translation2::from(self.transform.translation.vector + diff);
    }

    pub fn set_transform_position(
        &mut self,
        position: Vector2<f32>,
        old_pos: Vector2<f32>,
        rotation: f32,
    ) {
        let rotation = rotation + PI / 2.0;
        let direction = Vector2::new(rotation.cos(), rotation.sin());
        let direction = direction * 120.0 + (old_pos - position) * 0.0;
        let pos = position - (CAM_CENTER + direction);
        self.transform_next.translation = Translation2::from(pos);
    }
}
