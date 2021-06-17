use crate::PhysicsSystem;
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

    pub fn render(&mut self, ctx: &mut Context, physics: &mut PhysicsSystem) -> GameResult<()> {
        self.renderer.render(ctx, self, physics);
        self.render_color = ggez::graphics::WHITE;
        Ok(())
    }
}

impl Renderable for Destroyable {
    fn position(&self, physics: &mut PhysicsSystem) -> ggez::nalgebra::Point2<f32> {
        ggez::nalgebra::Point2::new(self.transform.position.x, self.transform.position.y)
    }
    fn rotation(&self, physics: &PhysicsSystem) -> f32 {
        self.transform.rotation
    }
    fn color(&self) -> ggez::graphics::Color {
        self.render_color
    }
}

impl Collidable for Destroyable {
    fn process_overlap(&mut self, _mpv: Vector2<f32>) {
        self.render_color = ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0);
    }
    fn process_collision(&mut self, _other: &impl Collidable, _n: &Vector2<f32>, _t: f32) {
        self.render_color = ggez::graphics::Color::new(1.0, 0.0, 0.0, 1.0);
    }
    fn position(&self) -> Isometry2<f32> {
        Isometry2::new(self.body.position, 0.0)
    }
    fn collider(&self) -> Polyline<f32> {
        let points = vec![
            Point2::new(14.0, 14.0),
            Point2::new(14.0, -14.0),
            Point2::new(-14.0, -14.0),
            Point2::new(-14.0, 14.0),
            Point2::new(14.0, 14.0),
        ];
        Polyline::new(points, Option::None)
    }
    fn velocity(&self) -> Vector2<f32> {
        self.body.velocity
    }
}
