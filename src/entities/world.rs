use ggez::graphics::DrawMode;
use ggez::graphics::{draw, Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::Point2;
use ggez::Context;
use rapier2d::prelude::*;

use crate::PhysicsSystem;

const CENTER: (f32, f32) = (400.0, 300.0);
const RADIUS: f32 = 300.0;
const PULL_FORCE: f32 = 1000.0;

pub struct World {
    mesh: Mesh,
}

impl World {
    pub fn new(ctx: &mut Context) -> Self {
        World {
            mesh: MeshBuilder::new()
                .circle(
                    DrawMode::stroke(1.0),
                    Point2::new(CENTER.0, CENTER.1),
                    RADIUS,
                    2.0,
                    WHITE,
                )
                .build(ctx)
                .expect("World generation failed."),
        }
    }

    pub fn update(&self, handle: &RigidBodyHandle, physics: &mut PhysicsSystem) {
        let body = physics.rigid_body_set.get_mut(*handle).unwrap();
        let direction = Vector::new(CENTER.0, CENTER.1) - body.translation();
        if direction.norm() > RADIUS {
            let direction = direction.normalize();
            body.apply_force(direction * PULL_FORCE, true);
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        draw(ctx, &self.mesh, (Point2::new(0.0, 0.0), 0.0, WHITE)).unwrap();
    }
}

pub trait BoundedByWorld {
    fn body_handle(&self) -> &RigidBodyHandle;
}
