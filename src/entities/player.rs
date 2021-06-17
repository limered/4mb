use crate::MyGame;
use crate::PhysicsSystem;
use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::prelude::*;

use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, GameResult};
use nalgebra as na;

use crate::entities::boost::Boost;
use crate::entities::player_mesh_creator;
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
    direction: na::Rotation2<f32>,
    main_renderer: LineMeshRenderer,
    state: PlayerState,
    boost: Boost,
    body_handle: RigidBodyHandle,
}

impl Player {
    pub fn new(ctx: &mut Context, game: &mut MyGame) -> Self {
        let rb = RigidBodyBuilder::new_dynamic()
            .translation(na::Vector2::new(300.0, 400.0))
            .additional_mass(1.0)
            .linear_damping(0.999)
            .can_sleep(false)
            .build();
        let handle = game.physic_system.rigid_body_set.insert(rb);
        Player {
            direction: na::Rotation2::new(0.0),
            state: PlayerState::Sliding,
            main_renderer: LineMeshRenderer::new(player_mesh_creator::create_player_mesh(
                &PLAYER_POINTS,
                ctx,
            )),
            boost: Boost::new(ctx, handle),
            body_handle: handle,
        }
    }

    pub fn update(&mut self, dt: f32, ctx: &Context, physics: &mut PhysicsSystem) {
        self.control_movement(ctx, physics);
        self.control_rotation(dt, ctx, physics);
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
            self.state = PlayerState::Boosting;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            movement = PLAYER_ACC;
            self.state = PlayerState::Boosting;
        } else {
            self.state = PlayerState::Sliding;
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
