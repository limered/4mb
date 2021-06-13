use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};

use crate::physic;

const _RED: graphics::Color = graphics::Color::new(1.0, 0.0, 0.0, 1.0);

pub struct Destroyable {
    pub transform: physic::Transform,
    pub body: physic::Body,
    pub mesh: graphics::Mesh,
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

// impl Collidable for Destroyable {
//     fn process_overlap(&mut self, _mpv: Vector2<f32>) {
//         self.render_color = RED;
//     }
//     fn process_collision(&mut self, other: &impl Collidable, n: &Vector2<f32>, t: f32){

//     }

//     fn collider(&self) -> PolyCollider {
//         self.collider.clone()
//     }
//     fn velocity(&self) -> Vector2<f32> {
//         self.body.velocity.clone()
//     }
// }
