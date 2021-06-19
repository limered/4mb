use ggez::Context;
use ggez::GameResult;
use nalgebra as na;
use nalgebra::Vector2;
use rand::Rng;
use rapier2d::prelude::*;

use crate::systems::render_system::line_mesh::LineMeshRenderer;
use crate::systems::render_system::Renderable;
use crate::PhysicsSystem;

pub struct EnemySystem {
    pub enemies: Vec<Enemy>,
}

impl EnemySystem {
    pub fn new() -> Self {
        EnemySystem {
            enemies: Vec::new(),
        }
    }

    pub fn make_enemy(&mut self, ctx: &mut Context, physics: &mut PhysicsSystem) {
        let enemy = Enemy::new(ctx, physics);
        self.enemies.push(enemy);
    }

    pub fn update(&mut self, dt: f32, ctx: &Context, physics: &mut PhysicsSystem) {
        for enemy in self.enemies.iter_mut() {
            enemy.update(dt, ctx, physics);
        }
    }

    pub fn render(&mut self, ctx: &mut Context, physics: &mut PhysicsSystem) {
        for enemy in self.enemies.iter_mut() {
            enemy.render(ctx, physics).unwrap();
        }
    }
}

pub struct Enemy {
    _movement_direction: na::Vector2<f32>,
    main_renderer: LineMeshRenderer,
    body_handle: RigidBodyHandle,
    _collider_handle: ColliderHandle,
}

fn random_vec(x: (f32, f32), y: (f32, f32)) -> nalgebra::Point2<f32> {
    let mut rng = rand::thread_rng();
    let x: f32 = (rng.gen::<f32>() * x.1) + x.0;
    let y: f32 = (rng.gen::<f32>() * y.1) + y.0;
    nalgebra::Point2::new(x, y)
}

const MAX_SIZE: f32 = 10.0;
const MIN_SIZE: f32 = 10.0;

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

const MASS: f32 = 1.0;

impl Enemy {
    pub fn new(ctx: &mut Context, physic_system: &mut PhysicsSystem) -> Self {
        let points = create_mesh_points();
        let rb = RigidBodyBuilder::new_dynamic()
            .translation(na::Vector2::new(200.0, 100.0))
            .linvel(Vector2::new(5.0, 5.0))
            .additional_mass(MASS)
            .additional_principal_angular_inertia(MASS)
            .can_sleep(false)
            .ccd_enabled(true)
            .build();
        let body_handle = physic_system.rigid_body_set.insert(rb);
        let collider = ColliderBuilder::polyline(points.to_vec(), Option::None)
            .density(0.0)
            .build();
        let collider_handle = physic_system.collider_set.insert_with_parent(
            collider,
            body_handle,
            &mut physic_system.rigid_body_set,
        );
        Enemy {
            body_handle,
            _collider_handle: collider_handle,
            _movement_direction: na::Vector2::new(400.0, 300.0) - na::Vector2::new(200.0, 100.0),
            main_renderer: LineMeshRenderer::new(create_mesh(ctx, points)),
        }
    }

    pub fn update(&mut self, _dt: f32, _ctx: &Context, _physics: &mut PhysicsSystem) {
        // pull into the middle

        // let body = physics.rigid_body_set.get_mut(self.body_handle).unwrap();
        // body.apply_force(force: Vector<Real>, wake_up: bool)
    }

    pub fn render(&mut self, ctx: &mut Context, physics: &mut PhysicsSystem) -> GameResult<()> {
        self.main_renderer.render(ctx, self, physics);

        Ok(())
    }
}

impl Renderable for Enemy {
    fn position(&self, physics: &mut PhysicsSystem) -> ggez::nalgebra::Point2<f32> {
        let body = physics.rigid_body_set.get(self.body_handle).unwrap();
        ggez::nalgebra::Point2::new(body.translation().x, body.translation().y)
    }
    fn rotation(&self, physics: &PhysicsSystem) -> f32 {
        let body = physics.rigid_body_set.get(self.body_handle).unwrap();
        body.rotation().angle()
    }
    fn color(&self) -> ggez::graphics::Color {
        ggez::graphics::WHITE
    }
}
