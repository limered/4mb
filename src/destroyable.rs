use crate::PhysicsSystem;
use ggez::{graphics, Context, GameResult};
use nalgebra::Vector2;

use crate::render_system::line_mesh::LineMeshRenderer;
use crate::render_system::Renderable;

const _RED: graphics::Color = graphics::Color::new(1.0, 0.0, 0.0, 1.0);

fn create_cube_mesh(ctx: &mut Context) -> ggez::graphics::Mesh {
    let mesh_points: [ggez::nalgebra::Point2<f32>; 5] = [
        ggez::nalgebra::Point2::new(15.0, 15.0),
        ggez::nalgebra::Point2::new(15.0, -15.0),
        ggez::nalgebra::Point2::new(-15.0, -15.0),
        ggez::nalgebra::Point2::new(-15.0, 15.0),
        ggez::nalgebra::Point2::new(15.0, 15.0),
    ];
    ggez::graphics::MeshBuilder::new()
        .line(&mesh_points, 2.0, ggez::graphics::WHITE)
        .unwrap()
        .build(ctx)
        .expect("Could not build PLayer Mesh")
}

pub struct Destroyable {
    pub renderer: LineMeshRenderer,
    pub render_color: graphics::Color,
}

impl Destroyable {
    pub fn new(_position: Vector2<f32>, ctx: &mut Context) -> Self {
        Destroyable {
            renderer: LineMeshRenderer::new(create_cube_mesh(ctx)),
            render_color: graphics::WHITE,
        }
    }

    pub fn render(&mut self, ctx: &mut Context, physics: &mut PhysicsSystem) -> GameResult<()> {
        self.renderer.render(ctx, self, physics);
        self.render_color = ggez::graphics::WHITE;
        Ok(())
    }
}

impl Renderable for Destroyable {
    fn position(&self, _physics: &mut PhysicsSystem) -> ggez::nalgebra::Point2<f32> {
        ggez::nalgebra::Point2::new(0.0, 0.0)
    }
    fn rotation(&self, _physics: &PhysicsSystem) -> f32 {
        0.0
    }
    fn color(&self) -> ggez::graphics::Color {
        self.render_color
    }
}
