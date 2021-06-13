use ggez::{graphics, Context, GameResult};
use nalgebra::{Isometry2, Point2, Vector2};
use ncollide2d::shape::Polyline;

use crate::collision_system::Collidable;
use crate::render_system::line_mesh::LineMeshRenderer;
use crate::render_system::Renderable;

use crate::physic;

const _RED: graphics::Color = graphics::Color::new(1.0, 0.0, 0.0, 1.0);

fn create_cube_mesh(ctx: &mut Context) -> ggez::graphics::Mesh {
    let mesh_points: [ggez::nalgebra::Point2<f32>; 5] = [
        ggez::nalgebra::Point2::new(15.0, 15.0),
        ggez::nalgebra::Point2::new(15.0, -15.0),
        ggez::nalgebra::Point2::new(-15.0, -15.0),
        ggez::nalgebra::Point2::new(-15.0, 15.0),
        ggez::nalgebra::Point2::new(15.0, 15.0),
    ];
    ggez::graphics::MeshBuilder::new()
        .line(&mesh_points, 2.0, ggez::graphics::WHITE)
        .unwrap()
        .build(ctx)
        .expect("Could not build PLayer Mesh")
}

pub struct Destroyable {
    pub transform: physic::Transform,
    pub body: physic::Body,
    pub renderer: LineMeshRenderer,
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
            renderer: LineMeshRenderer::new(create_cube_mesh(ctx)),
            render_color: graphics::WHITE,
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.renderer.render(ctx, self);
        self.render_color = ggez::graphics::WHITE;
        Ok(())
    }
}

impl Renderable for Destroyable {
    fn position(&self) -> ggez::nalgebra::Point2<f32> {
        ggez::nalgebra::Point2::new(self.transform.position.x, self.transform.position.y)
    }
    fn rotation(&self) -> f32 {
        self.transform.rotation
    }
    fn color(&self) -> ggez::graphics::Color {
        self.render_color
    }
}

//     }

//     fn collider(&self) -> PolyCollider {
//         self.collider.clone()
//     }
//     fn velocity(&self) -> Vector2<f32> {
//         self.body.velocity.clone()
//     }
// }
