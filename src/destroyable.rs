use crate::collision_system::{sat, Collidable};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};

use crate::collision_system::colliders::poly_collider::PolyCollider;
use crate::physic;

const RED: graphics::Color = graphics::Color::new(1.0, 0.0, 0.0, 1.0);

pub struct Destroyable {
    pub transform: physic::Transform,
    pub body: physic::Body,
    pub mesh: graphics::Mesh,
    pub collider: PolyCollider,
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
            collider: PolyCollider::new(
                Vector2::new(position.x, position.y),
                vec![
                    Vector2::new(15.0, 15.0),
                    Vector2::new(15.0, -15.0),
                    Vector2::new(-15.0, -15.0),
                    Vector2::new(-15.0, 15.0),
                ],
            ),
            render_color: graphics::WHITE,
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

impl Collidable for Destroyable {
    fn collide(&mut self, other: &mut impl Collidable) -> (bool, Vector2<f32>) {
        sat::is_collision(self.collider(), other.collider())
    }
    fn respond(&mut self, _mpv: Vector2<f32>) {
        self.render_color = RED;
    }

    fn collider(&self) -> &PolyCollider {
        &self.collider
    }
}
