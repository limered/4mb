use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};

use crate::physic;

pub struct Destroyable {
    pub transform: physic::Transform,
    pub body: physic::Body,
    pub mesh: graphics::Mesh,
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
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult<()> {
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
}
