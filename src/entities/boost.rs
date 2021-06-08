use ggez::nalgebra::Point2;
use ggez::Context;
use rand::Rng;

use crate::entities::player_mesh_creator;
use crate::physic::Transform;
use crate::render_system::line_mesh::LineMeshRenderer;
use crate::render_system::Renderable;

pub struct Boost {
    transform: Transform,
    renderer: LineMeshRenderer,
}

impl Boost {
    pub fn new(ctx: &mut Context) -> Self {
        Boost {
            transform: Transform::new(Point2::new(0.0, 0.0), 0.0),
            renderer: LineMeshRenderer::new(player_mesh_creator::create_player_boost_mesh(ctx)),
        }
    }
    pub fn update(&mut self, transform: &Transform) {
        self.transform.position = transform.position;
        self.transform.rotation = transform.rotation;
    }

    pub fn render(&self, ctx: &mut Context) {
        self.renderer.render(ctx, self);
    }
}

impl Renderable for Boost {
    fn position(&self) -> Point2<f32> {
        let mut rng = rand::thread_rng();
        let x_rng: f32 = rng.gen();
        let x_rng = (x_rng * 6.0) - 3.0;
        let y_rng: f32 = rng.gen();
        let y_rng = (y_rng * 6.0) - 3.0;
        Point2::new(
            self.transform.position.x + x_rng,
            self.transform.position.y + y_rng,
        )
    }
    fn rotation(&self) -> f32 {
        self.transform.rotation
    }
}
