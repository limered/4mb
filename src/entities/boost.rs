use ggez::Context;
use rand::Rng;
use rapier2d::prelude::*;

use crate::entities::player_mesh_creator;
use crate::systems::render_system::line_mesh::LineMeshRenderer;
use crate::systems::render_system::Renderable;
use crate::PhysicsSystem;

pub struct Boost {
    renderer: LineMeshRenderer,
    player_body_handle: RigidBodyHandle,
}

impl Boost {
    pub fn new(ctx: &mut Context, player_body_handle: RigidBodyHandle) -> Self {
        Boost {
            renderer: LineMeshRenderer::new(player_mesh_creator::create_player_boost_mesh(ctx)),
            player_body_handle,
        }
    }

    pub fn render(&self, ctx: &mut Context, physics: &mut PhysicsSystem) {
        self.renderer.render(ctx, self, physics);
    }
}

impl Renderable for Boost {
    fn position(&self, physics: &mut PhysicsSystem) -> ggez::nalgebra::Point2<f32> {
        let body = physics.rigid_body_set.get(self.player_body_handle).unwrap();
        let mut rng = rand::thread_rng();
        let x_rng: f32 = rng.gen();
        let x_rng = (x_rng * 6.0) - 3.0;
        let y_rng: f32 = rng.gen();
        let y_rng = (y_rng * 6.0) - 3.0;
        ggez::nalgebra::Point2::new(body.translation().x + x_rng, body.translation().y + y_rng)
    }
    fn rotation(&self, physics: &PhysicsSystem) -> f32 {
        let body = physics.rigid_body_set.get(self.player_body_handle).unwrap();
        body.rotation().angle()
    }
    fn color(&self) -> ggez::graphics::Color {
        ggez::graphics::WHITE
    }
}
