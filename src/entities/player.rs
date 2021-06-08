use crate::render_system::line_mesh::LineMeshRenderer;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra::{self as na, Point2, Vector2};
use ggez::{Context, GameResult};

use crate::collision_system::colliders::poly_collider::PolyCollider;
use crate::collision_system::{sat, Collidable, Rotatable};
use crate::entities::boost::Boost;
use crate::entities::player_mesh_creator;
use crate::physic;
use crate::render_system::Renderable;

const PLAYER_ACC: f32 = 800.0;
const PLAYER_TURN: f32 = 4.0;
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
    pub collider: PolyCollider,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Self {
        let transform = physic::Transform::new(Point2::new(0.0, 0.0), 0.0);
        Player {
            body: physic::Body::new(Vector2::new(300.0, 400.0), 0.99),
            transform,
            direction: na::Rotation2::new(0.0),
            state: PlayerState::Sliding,
            collider: PolyCollider::new(
                Vector2::new(0.0, 0.0),
                PLAYER_POINTS
                    .iter()
                    .map(|p| Vector2::new(p.0, p.1))
                    .collect(),
            ),
            main_renderer: LineMeshRenderer::new(player_mesh_creator::create_player_mesh(
                &PLAYER_POINTS,
                ctx,
            )),
            boost: Boost::new(ctx),
        }
    }

    pub fn update(&mut self, dt: f32, ctx: &Context) {
        self.control_movement(ctx);
        self.control_rotation(dt, ctx);

        self.check_bounds();
        self.update_state();

        self.body.animate(dt);
        self.transform.position = Point2::new(self.body.position.x, self.body.position.y);

        self.update_collider();
        self.update_boost();
    }

    fn update_boost(&mut self) {
        self.boost.update(&self.transform);
    }

    fn update_collider(&mut self) {
        self.collider.rotate(&self.direction);
        self.collider.center = self.body.position.clone();
    }

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

    pub fn render(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.main_renderer.render(ctx, self);
        // self.collider.draw(ctx);

        match self.state {
            PlayerState::Boosting => {
                self.boost.render(ctx);
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

    fn control_rotation(&mut self, dt: f32, ctx: &Context) {
        let mut r = na::Rotation2::new(0.0);
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            r = na::Rotation2::new(PLAYER_TURN * dt);
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            r = na::Rotation2::new(-PLAYER_TURN * dt);
        }
        self.direction = r * self.direction;
        self.transform.rotation = self.direction.angle();
    }

    fn control_movement(&mut self, ctx: &Context) {
        let mut movement: f32 = 0.0;
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            movement = -PLAYER_ACC;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            movement = PLAYER_ACC;
        }
        self.body.acceleration = self.direction * Vector2::new(0.0, movement);
    }
}

impl Renderable for Player {
    fn position(&self) -> Point2<f32> {
        self.transform.position
    }
    fn rotation(&self) -> f32 {
        self.transform.rotation
    }
}

impl Collidable for Player {
    fn collide(&mut self, other: &mut impl Collidable) -> (bool, Vector2<f32>) {
        sat::is_collision(self.collider(), other.collider())
    }
    fn respond(&mut self, mpv: Vector2<f32>) {
        self.body.stop();
        self.body.acceleration = -mpv;
    }
    fn collider(&self) -> &PolyCollider {
        &self.collider
    }
}
