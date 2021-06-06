use ggez::nalgebra as na;
use na::{Rotation2, Vector2};

pub mod colliders;
pub mod sat;

pub trait Rotatable {
    fn rotate(&mut self, rotation: &Rotation2<f32>);
}

pub trait PolyCollision {
    fn edges_of(&self) -> Vec<Vector2<f32>>;
    fn vertices(&self) -> Vec<Vector2<f32>>;
    fn collide(&self, other: &impl PolyCollision) -> bool;
}
