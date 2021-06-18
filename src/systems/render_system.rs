use crate::PhysicsSystem;
use ggez::nalgebra::Point2;

pub mod line_mesh;

pub trait Renderable {
    fn position(&self, physics: &mut PhysicsSystem) -> Point2<f32>;
    fn rotation(&self, physics: &PhysicsSystem) -> f32;
    fn color(&self) -> ggez::graphics::Color;
}
