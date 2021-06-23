use ggez::graphics::WHITE;
use ggez::Context;
use nalgebra::Vector2;
use rand::Rng;
use rapier2d::prelude::*;

use crate::constants::*;
use crate::systems::render_system::RenderInfo;
use crate::PhysicsSystem;
use crate::Renderable;

#[derive(PartialEq, Clone, Copy)]
pub enum EnemySize {
    Big,
    Middle,
    Small,
}

#[derive(PartialEq)]
pub struct Enemy {
    pub collider_handle: ColliderHandle,
    pub body_handle: RigidBodyHandle,
    pub size: EnemySize,
    pub life_time: f32,
    _movement_direction: Vector2<f32>,
    render_info: RenderInfo,
}

impl Enemy {
    pub fn new(
        ctx: &mut Context,
        position: Vector2<f32>,
        physic_system: &mut PhysicsSystem,
        size: EnemySize,
    ) -> Self {
        let points = create_mesh_points(size.clone());
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
            collider_handle,
            _movement_direction: Vector2::new(400.0, 300.0) - Vector2::new(200.0, 100.0),
            render_info: RenderInfo::new(create_mesh(ctx, points), WHITE),
            life_time: ENEMY_MAX_LIFETIME,
            size,
        }
    }

    pub fn update(&mut self, dt: f32, _ctx: &Context, physics: &mut PhysicsSystem) {
        let body = physics.rigid_body_set.get_mut(self.body_handle).unwrap();
        let position = body.translation().clone();
        let direction = Vector2::new(MIDDLE.0, MIDDLE.1) - position;
        let direction = direction.normalize();
        body.apply_force(direction * EARTH_GRAVITY, true);

        self.render_info
            .update(self.position(physics), self.rotation(physics), self.color());

        if self.size == EnemySize::Small {
            let distance_to_earth = (Vector::new(MIDDLE.0, MIDDLE.1) - position).norm();
            if distance_to_earth < ENEMY_BURNUP_DISTANCE {
                self.life_time -= dt;
                let scale = self.life_time / ENEMY_MAX_LIFETIME;
                self.render_info.set_scale(scale);
            }
        }
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
    fn info_as_ref(&self) -> Vec<RenderInfo> {
        vec![self.render_info.clone()]
    }
}

fn create_mesh_points(size: EnemySize) -> [nalgebra::Point2<f32>; 5] {
    let mesh_points: [nalgebra::Point2<f32>; 5] = match size {
        EnemySize::Big => {
            let min = ENEMY_MIN_SIZE;
            let max = ENEMY_MAX_SIZE;
            let p1 = random_vec((min, max), (min, max));
            [
                p1,
                random_vec((-min, -max), (min, max)),
                random_vec((-min, -max), (-min, -max)),
                random_vec((min, max), (-min, -max)),
                p1,
            ]
        }
        EnemySize::Middle => {
            let min = ENEMY_MIN_SIZE / 2.0;
            let max = ENEMY_MAX_SIZE / 2.0;
            let p1 = random_vec((min, max), (min, max));
            [
                p1,
                random_vec((-min, -max), (min, max)),
                random_vec((-min, -max), (-min, -max)),
                random_vec((min, max), (-min, -max)),
                p1,
            ]
        }
        EnemySize::Small => {
            let min = ENEMY_MIN_SIZE / 4.0;
            let max = ENEMY_MAX_SIZE / 4.0;
            let p1 = random_vec((min, max), (min, max));
            [
                p1,
                random_vec((-min, -max), (min, max)),
                random_vec((-min, -max), (-min, -max)),
                random_vec((min, max), (-min, -max)),
                p1,
            ]
        }
    };
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
