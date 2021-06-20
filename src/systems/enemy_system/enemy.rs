use ggez::graphics::WHITE;
use ggez::Context;
use nalgebra::Vector2;
use rand::Rng;
use rapier2d::prelude::*;

use crate::systems::enemy_system::MIDDLE;
use crate::systems::render_system::RenderInfo;
use crate::PhysicsSystem;
use crate::Renderable;

const SPEED: f32 = 90.0;
const MAX_SIZE: f32 = 10.0;
const MIN_SIZE: f32 = 10.0;

#[derive(PartialEq)]
pub struct Enemy {
    _movement_direction: Vector2<f32>,
    render_info: RenderInfo,
    body_handle: RigidBodyHandle,
    _collider_handle: ColliderHandle,
}

impl Enemy {
    pub fn new(
        ctx: &mut Context,
        position: Vector2<f32>,
        physic_system: &mut PhysicsSystem,
    ) -> Self {
        let points = create_mesh_points();
        let rb = RigidBodyBuilder::new_dynamic()
            .translation(position)
            .can_sleep(false)
            .ccd_enabled(false)
            .build();
        let body_handle = physic_system.rigid_body_set.insert(rb);

        let collider = ColliderBuilder::convex_hull(&points.to_vec())
            .unwrap()
            .density(0.1)
            .build();
        let collider_handle = physic_system.collider_set.insert_with_parent(
            collider,
            body_handle,
            &mut physic_system.rigid_body_set,
        );
        Enemy {
            body_handle,
            _collider_handle: collider_handle,
            _movement_direction: Vector2::new(400.0, 300.0) - Vector2::new(200.0, 100.0),
            render_info: RenderInfo::new(create_mesh(ctx, points), WHITE),
        }
    }

    pub fn update(&mut self, _dt: f32, _ctx: &Context, physics: &mut PhysicsSystem) {
        let body = physics.rigid_body_set.get_mut(self.body_handle).unwrap();
        let direction = Vector2::new(MIDDLE.0, MIDDLE.1) - body.translation();
        let direction = direction.normalize();
        body.apply_force(direction * SPEED, true);

        self.render_info
            .update(self.position(physics), self.rotation(physics), self.color());
    }
}

impl Renderable for Enemy {
    fn position(&self, physics: &PhysicsSystem) -> Vector2<f32> {
        let body = physics.rigid_body_set.get(self.body_handle).unwrap();
        Vector2::new(body.translation().x, body.translation().y)
    }
    fn rotation(&self, physics: &PhysicsSystem) -> f32 {
        let body = physics.rigid_body_set.get(self.body_handle).unwrap();
        body.rotation().angle()
    }
    fn color(&self) -> ggez::graphics::Color {
        ggez::graphics::WHITE
    }
    fn info_as_ref(&self) -> RenderInfo {
        self.render_info.clone()
    }
}

fn create_mesh_points() -> [nalgebra::Point2<f32>; 5] {
    let p1 = random_vec((MIN_SIZE, MAX_SIZE), (MIN_SIZE, MAX_SIZE));
    let mesh_points: [nalgebra::Point2<f32>; 5] = [
        p1,
        random_vec((-MIN_SIZE, -MAX_SIZE), (MIN_SIZE, MAX_SIZE)),
        random_vec((-MIN_SIZE, -MAX_SIZE), (-MIN_SIZE, -MAX_SIZE)),
        random_vec((MIN_SIZE, MAX_SIZE), (-MIN_SIZE, -MAX_SIZE)),
        p1,
    ];
    mesh_points
}

fn create_mesh(ctx: &mut Context, points: [nalgebra::Point2<f32>; 5]) -> ggez::graphics::Mesh {
    let mesh_points: Vec<ggez::nalgebra::Point2<f32>> = points
        .iter()
        .map(|p| ggez::nalgebra::Point2::new(p.x, p.y))
        .collect();
    ggez::graphics::MeshBuilder::new()
        .line(&mesh_points, 1.0, ggez::graphics::WHITE)
        .unwrap()
        .build(ctx)
        .expect("Could not build Player Mesh")
}

fn random_vec(x: (f32, f32), y: (f32, f32)) -> nalgebra::Point2<f32> {
    let mut rng = rand::thread_rng();
    let x: f32 = (rng.gen::<f32>() * x.1) + x.0;
    let y: f32 = (rng.gen::<f32>() * y.1) + y.0;
    nalgebra::Point2::new(x, y)
}
