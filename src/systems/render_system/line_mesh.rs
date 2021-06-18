use crate::PhysicsSystem;
use ggez::graphics::{draw, Mesh};
use ggez::Context;

use crate::systems::render_system::Renderable;

pub struct LineMeshRenderer {
    mesh: Mesh,
}

impl LineMeshRenderer {
    pub fn new(mesh: Mesh) -> Self {
        LineMeshRenderer { mesh }
    }

    pub fn render(
        &self,
        ctx: &mut Context,
        renderable: &impl Renderable,
        physics: &mut PhysicsSystem,
    ) {
        draw(
            ctx,
            &self.mesh,
            (
                renderable.position(physics),
                renderable.rotation(physics),
                renderable.color(),
            ),
        )
        .unwrap();
    }
}
