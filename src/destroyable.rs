use crate::collision_system::PolyCollision;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};

use crate::collision_system::colliders::quad_collider::QuadCollider;
use crate::physic;

const RED: graphics::Color = graphics::Color::new(1.0, 0.0, 0.0, 1.0);

pub struct Destroyable {
    pub transform: physic::Transform,
    pub body: physic::Body,
    pub mesh: graphics::Mesh,
    pub collider: QuadCollider,
    pub render_color: graphics::Color,
}

impl Destroyable {
    pub fn new(position: Vector2<f32>, ctx: &mut Context) -> Self {
        Destroyable {
            transform: physic::Transform {
                position: Point2::new(position.x, position.y),
                rotation: 0.0,
            },
            body: physic::Body::new(position, 0.99),
            mesh: graphics::MeshBuilder::new()
                .line(
                    &[
                        Point2::new(15.0, 15.0),
                        Point2::new(15.0, -15.0),
                        Point2::new(-15.0, -15.0),
                        Point2::new(-15.0, 15.0),
                        Point2::new(15.0, 15.0),
                    ],
                    2.0,
                    graphics::WHITE,
                )
                .unwrap()
                .build(ctx)
                .unwrap(),
            collider: QuadCollider::new(
                Vector2::new(position.x, position.y),
                [
                    Vector2::new(15.0, 15.0),
                    Vector2::new(15.0, -15.0),
                    Vector2::new(-15.0, -15.0),
                    Vector2::new(-15.0, 15.0),
                ],
            ),
            render_color: graphics::WHITE,
        }
    }

    pub fn check_collision(&mut self, player: &impl PolyCollision) {
        if self.collider.collide(player) {
            self.render_color = RED;
        } else {
            self.render_color = graphics::WHITE;
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(
            ctx,
            &self.mesh,
            (
                self.transform.position,
                self.transform.rotation,
                self.render_color,
            ),
        )
    }
}
