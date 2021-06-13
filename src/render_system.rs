use ggez::nalgebra::Point2;

pub mod line_mesh;

pub trait Renderable {
    fn position(&self) -> Point2<f32>;
    fn rotation(&self) -> f32;
    fn color(&self) -> ggez::graphics::Color;
}
