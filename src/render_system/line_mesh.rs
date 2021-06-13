use ggez::graphics::{draw, Mesh};
use ggez::Context;

use crate::render_system::Renderable;

pub struct LineMeshRenderer {
    mesh: Mesh,
}

impl LineMeshRenderer {
    pub fn new(mesh: Mesh) -> Self {
        LineMeshRenderer { mesh }
    }

    pub fn render(&self, ctx: &mut Context, renderable: &impl Renderable) {
        draw(
            ctx,
            &self.mesh,
            (
                renderable.position(),
                renderable.rotation(),
                renderable.color(),
            ),
        )
        .unwrap();
    }
}
