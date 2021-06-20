use ggez::graphics::DrawMode;
use ggez::graphics::{MeshBuilder, WHITE};
use ggez::nalgebra::Point2;
use ggez::Context;
use nalgebra::Vector2;
use rapier2d::prelude::*;

use crate::systems::render_system::RenderInfo;
use crate::systems::render_system::Renderable;
use crate::PhysicsSystem;

const CENTER: (f32, f32) = (400.0, 300.0);
const RADIUS: f32 = 300.0;
const PULL_FORCE: f32 = 80000.0;

#[derive(PartialEq)]
pub struct World {
    render_info: RenderInfo,
}

impl World {
    pub fn new(ctx: &mut Context) -> Self {
        World {
            render_info: RenderInfo::new(
                MeshBuilder::new()
                    .circle(
                        DrawMode::stroke(1.0),
                        Point2::new(CENTER.0, CENTER.1),
                        RADIUS,
                        2.0,
                        WHITE,
                    )
                    .build(ctx)
                    .expect("World generation failed."),
                WHITE,
            ),
        }
    }

    pub fn update(&mut self, handle: &RigidBodyHandle, physics: &mut PhysicsSystem) {
        let body = physics.rigid_body_set.get_mut(*handle).unwrap();
        let direction = Vector::new(CENTER.0, CENTER.1) - body.translation();
        if direction.norm() > RADIUS {
            let direction = direction.normalize();
            body.apply_force(direction * PULL_FORCE, true);
        }

        self.render_info
            .update(self.position(physics), self.rotation(physics), self.color());
    }
}

pub trait BoundedByWorld {
    fn body_handle(&self) -> &RigidBodyHandle;
}

impl Renderable for World {
    fn position(&self, _: &PhysicsSystem) -> Vector2<f32> {
        Vector2::new(0.0, 0.0)
    }
    fn color(&self) -> ggez::graphics::Color {
        ggez::graphics::WHITE
    }
    fn rotation(&self, _: &PhysicsSystem) -> f32 {
        0.0
    }
    fn info_as_ref(&self) -> RenderInfo {
        self.render_info.clone()
    }
}
