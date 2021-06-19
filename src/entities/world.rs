use ggez::graphics::DrawMode;
use ggez::graphics::{draw, Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::Point2;
use ggez::Context;

pub struct World {
    mesh: Mesh,
}

impl World {
    pub fn new(ctx: &mut Context) -> Self {
        World {
            mesh: MeshBuilder::new()
                .circle(
                    DrawMode::stroke(1.0),
                    Point2::new(400.0, 300.0),
                    300.0,
                    5.0,
                    WHITE,
                )
                .build(ctx).expect("World generation failed."),
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        draw(ctx, &self.mesh, (Point2::new(0.0, 0.0), 0.0, WHITE)).unwrap();
    }
}

