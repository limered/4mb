use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra::{self as na, Point2, Vector2};
use ggez::{graphics, Context};

pub struct Player {
    pub rotation: f32,
    pub direction: na::Rotation2<f32>,
    pub position: Point2<f32>,
    pub triangle_mesh: graphics::Mesh,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Self {
        Player {
            rotation: 0.0,
            direction: na::Rotation2::new(0.0),
            position: Point2::new(300.0, 400.0),
            triangle_mesh: graphics::MeshBuilder::new()
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
        }
    }

    pub fn update(&mut self, dt: f32, ctx: &Context) {
        self.move_self(dt, ctx);
        self.rotate_self(dt, ctx);
    }

    fn rotate_self(&mut self, dt: f32, ctx: &Context) {
        let mut r = na::Rotation2::new(0.0);
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            r = na::Rotation2::new(1.0 * dt);
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            r = na::Rotation2::new(-1.0 * dt);
        }
        self.direction = r * self.direction;
        self.rotation = self.direction.angle();
    }

    fn move_self(&mut self, dt: f32, ctx: &Context) {
        let mut movement: f32 = 0.0;
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            movement = -100.0;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            movement = 100.0;
        }
        let delta = self.direction * Vector2::new(0.0, movement) * dt;
        self.position.y += delta.y;
        self.position.x += delta.x;
    }
}
