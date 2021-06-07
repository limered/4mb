use ggez::nalgebra as na;
use na::{Rotation2, Vector2};

use crate::collision_system::Rotatable;

pub struct CircleCollider {
    center: Vector2<f32>,
    radius: f32,
}

impl CircleCollider {
    pub fn new(c: Vector2<f32>, r: f32) -> Self {
        Self {
            center: c,
            radius: r,
        }
    }
}

impl Rotatable for CircleCollider {
    fn rotate(&mut self, _rotation: &Rotation2<f32>) {}
}
