use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra::{self as na, Point2, Vector2};
use ggez::{graphics, Context, GameResult};
use rand::Rng;

use crate::collision_system::TriangleCollider::TriangleCollider;
use crate::physic;

const PLAYER_ACC: f32 = 800.0;
const PLAYER_TURN: f32 = 4.0;

#[derive(Debug)]
enum PlayerState {
    Sliding,
    Boosting,
}

pub struct Player {
    transform: physic::Transform,
    body: physic::Body,
    direction: na::Rotation2<f32>,
    mesh: graphics::Mesh,
    state: PlayerState,
    boost_mesh: graphics::Mesh,
    collider: TriangleCollider,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Self {
        let transform = physic::Transform::new(Point2::new(0.0, 0.0), 0.0);
        Player {
            body: physic::Body::new(Vector2::new(300.0, 400.0), 0.99),
            transform,
            direction: na::Rotation2::new(0.0),
            state: PlayerState::Sliding,
            collider: TriangleCollider::new(
                Vector2::new(0.0, 0.0),
                [
                    Vector2::new(15.0, 15.0),
                    Vector2::new(0.0, -15.0),
                    Vector2::new(-15.0, 15.0),
                ],
            ),
            mesh: graphics::MeshBuilder::new()
                .line(
                    &[
                        Point2::new(15.0, 15.0),
                        Point2::new(0.0, -15.0),
                        Point2::new(-15.0, 15.0),
                    ],
                    2.0,
                    graphics::WHITE,
                )
                .unwrap()
                .build(ctx)
                .unwrap(),
            boost_mesh: graphics::MeshBuilder::new()
                .line(
                    &[Point2::new(-3.0, 15.0), Point2::new(-6.0, 23.0)],
                    2.0,
                    graphics::WHITE,
                )
                .unwrap()
                .line(
                    &[Point2::new(0.0, 15.0), Point2::new(0.0, 24.0)],
                    2.0,
                    graphics::WHITE,
                )
                .unwrap()
                .line(
                    &[Point2::new(3.0, 15.0), Point2::new(6.0, 23.0)],
                    2.0,
                    graphics::WHITE,
                )
                .unwrap()
                .build(ctx)
                .unwrap(),
        }
    }

    pub fn update(&mut self, dt: f32, ctx: &Context) {
        self.move_self(ctx);
        self.rotate_self(dt, ctx);
        self.update_collider();
        self.check_bounds();
        self.update_state();

        self.body.animate(dt);
        self.transform.position = Point2::new(self.body.position.x, self.body.position.y);
    }

    fn update_collider(&mut self) {
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
        match self.state {
            PlayerState::Boosting => {
                let mut rng = rand::thread_rng();
                let x_rng: f32 = rng.gen();
                let x_rng = (x_rng * 6.0) - 3.0;
                let y_rng: f32 = rng.gen();
                let y_rng = (y_rng * 6.0) - 3.0;
                let boost_position = Point2::new(
                    self.transform.position.x + x_rng,
                    self.transform.position.y + y_rng,
                );
                graphics::draw(
                    ctx,
                    &self.boost_mesh,
                    (boost_position, self.transform.rotation, graphics::WHITE),
                )
                .expect("Booster could not be drawn.");
            }
            _ => (),
        }

        self.collider.draw(ctx);

        graphics::draw(
            ctx,
            &self.mesh,
            (
                self.transform.position,
                self.transform.rotation,
                graphics::WHITE,
            ),
        )
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

    fn rotate_self(&mut self, dt: f32, ctx: &Context) {
        let mut r = na::Rotation2::new(0.0);
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            r = na::Rotation2::new(PLAYER_TURN * dt);
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            r = na::Rotation2::new(-PLAYER_TURN * dt);
        }
        self.direction = r * self.direction;
        self.transform.rotation = self.direction.angle();
    }

    fn move_self(&mut self, ctx: &Context) {
        let mut movement: f32 = 0.0;
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            movement = -PLAYER_ACC;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            movement = PLAYER_ACC;
        }
        self.body.acceleration = self.direction * Vector2::new(0.0, movement);
    }
}
