use crate::MyGame;
use crate::PhysicsSystem;
use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::prelude::*;

use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, GameResult};
use nalgebra as na;
use ncollide2d::shape::Polyline;

use crate::collision_system::Collidable;
use crate::entities::boost::Boost;
use crate::entities::player_mesh_creator;
use crate::physic;
use crate::render_system::line_mesh::LineMeshRenderer;
use crate::render_system::Renderable;

const PLAYER_ACC: f32 = 800.0;
const PLAYER_TURN: f32 = 4.5;
const PLAYER_POINTS: [(f32, f32); 3] = [(15.0, 15.0), (0.0, -15.0), (-15.0, 15.0)];

#[derive(Debug)]
enum PlayerState {
    Sliding,
    Boosting,
}

pub struct Player {
    pub transform: physic::Transform,
    body: physic::Body,
    direction: na::Rotation2<f32>,
    main_renderer: LineMeshRenderer,
    state: PlayerState,
    boost: Boost,
    body_handle: RigidBodyHandle,
}

impl Player {
    pub fn new(ctx: &mut Context, game: &mut MyGame) -> Self {
        let transform = physic::Transform::new(na::Point2::new(0.0, 0.0), 0.0);
        let rb = RigidBodyBuilder::new_dynamic()
            .translation(na::Vector2::new(300.0, 400.0))
            .additional_mass(1.0)
            .linear_damping(0.999)
            .can_sleep(false)
            .build();
        Player {
            body: physic::Body::new(na::Vector2::new(300.0, 400.0), 0.99),
            transform,
            direction: na::Rotation2::new(0.0),
            state: PlayerState::Sliding,
            main_renderer: LineMeshRenderer::new(player_mesh_creator::create_player_mesh(
                &PLAYER_POINTS,
                ctx,
            )),
            boost: Boost::new(ctx),
            body_handle: game.physic_system.rigid_body_set.insert(rb),
        }
    }

    pub fn update(&mut self, dt: f32, ctx: &Context, physics: &mut PhysicsSystem) {
        self.control_movement(ctx, physics);
        self.control_rotation(dt, ctx, physics);

        self.check_bounds();
        self.update_state();

        self.update_collider();
        self.update_boost();
    }

    fn update_boost(&mut self) {
        self.boost.update(&self.transform);
    }

    fn update_collider(&mut self) {}

    fn update_state(&mut self) {
        match self.state {
            PlayerState::Boosting => {
                if self.body.acceleration.norm() <= 0.0 {
                    self.state = PlayerState::Sliding;
                }
            }
            PlayerState::Sliding => {
                if self.body.acceleration.norm() > 0.0 {
                    self.state = PlayerState::Boosting;
                }
            }
        }
    }

    pub fn render(&mut self, ctx: &mut Context, physics: &mut PhysicsSystem) -> GameResult<()> {
        self.main_renderer.render(ctx, self, physics);

        match self.state {
            PlayerState::Boosting => {
                self.boost.render(ctx, physics);
            }
            _ => (),
        };

        Ok(())
    }

    fn check_bounds(&mut self) {
        let pos = self.body.position;
        if pos.x < 0.0 {
            self.body.stop();
            self.body.position.x = 0.0;
        } else if pos.x > 800.0 {
            self.body.stop();
            self.body.position.x = 800.0;
        }
        if pos.y < 0.0 {
            self.body.stop();
            self.body.position.y = 0.0;
        } else if pos.y > 600.0 {
            self.body.stop();
            self.body.position.y = 600.0;
        }
    }

    fn control_rotation(&mut self, dt: f32, ctx: &Context, physics: &mut PhysicsSystem) {
        let mut r = na::Rotation2::new(0.0);
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            r = na::Rotation2::new(PLAYER_TURN * dt);
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            r = na::Rotation2::new(-PLAYER_TURN * dt);
        }
        let body = physics.rigid_body_set.get_mut(self.body_handle).unwrap();
        self.direction = r * self.direction;
        body.set_rotation(self.direction.angle(), true)
    }

    fn control_movement(&mut self, ctx: &Context, physics: &mut PhysicsSystem) {
        let mut movement: f32 = 0.0;
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            movement = -PLAYER_ACC;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            movement = PLAYER_ACC;
        }

        let force = self.direction * vector![0.0, movement];
        let body = physics.rigid_body_set.get_mut(self.body_handle).unwrap();
        body.apply_force(force, true);
    }
}

impl Renderable for Player {
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

impl Collidable for Player {
    fn process_overlap(&mut self, _mpv: na::Vector2<f32>) {}
    fn process_collision(&mut self, _other: &impl Collidable, _n: &na::Vector2<f32>, _t: f32) {}
    fn position(&self) -> na::Isometry2<f32> {
        na::Isometry2::new(self.body.position, 0.0)
    }
    fn collider(&self) -> Polyline<f32> {
        let mut points: Vec<na::Point2<f32>> = PLAYER_POINTS
            .iter()
            .map(|p| na::Point2::new(p.0, p.1))
            .collect();
        points.push(na::Point2::new(PLAYER_POINTS[0].0, PLAYER_POINTS[0].1));
        Polyline::new(points, Option::None)
    }
    fn velocity(&self) -> na::Vector2<f32> {
        self.body.velocity
    }
}
