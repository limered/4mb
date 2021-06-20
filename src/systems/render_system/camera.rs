use nalgebra::Isometry2;
use nalgebra::Point2;
use nalgebra::Translation2;
use nalgebra::Vector2;

const CAM_CENTER: Vector2<f32> = Vector2::new(400.0, 300.0);

pub struct Camera {
    pub transform: Isometry2<f32>,
}

impl Camera {
    pub fn new(transform: Isometry2<f32>) -> Self {
        Camera { transform }
    }

    pub fn modify(&self, vec: Vector2<f32>) -> Point2<f32> {
        let p = Point2::new(vec.x, vec.y);
        self.transform.inverse().transform_point(&p)
    }

    pub fn set_transform_position(&mut self, vec: Vector2<f32>) {
        let pos = vec - CAM_CENTER;
        self.transform.translation = Translation2::from(pos);
    }
}
