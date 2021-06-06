use crate::collision_system::{is_collision, PolyCollision, Rotatable};

use ggez::nalgebra as na;
use ggez::{graphics, Context};
use na::{Point2, Rotation2, Vector2};

pub struct TriangleCollider {
    pub center: Vector2<f32>,
    pub vertices: [Vector2<f32>; 3],
}

impl TriangleCollider {
    pub fn new(center: Vector2<f32>, vertices: [Vector2<f32>; 3]) -> Self {
        Self { center, vertices }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let mut verts: Vec<Point2<f32>> = self
            .vertices()
            .iter()
            .map(|v| Point2::new(v.x, v.y))
            .collect();
        verts.push(verts[0].clone());
        let mesh = graphics::MeshBuilder::new()
            .line(&verts, 2.0, graphics::WHITE)
            .unwrap()
            .build(ctx)
            .unwrap();

        graphics::draw(
            ctx,
            &mesh,
            (
                Point2::new(0.0, 0.0),
                0.0,
                graphics::Color::new(1.0, 0.0, 0.0, 1.0),
            ),
        )
        .expect("cant draw collider");
    }
}

impl Rotatable for TriangleCollider {
    fn rotate(&self, rotation: Rotation2<f32>) -> Self {
        let mut vertices = [Vector2::x(); 3];
        for (i, vertex) in self.vertices.iter().enumerate() {
            vertices[i] = rotation.transform_vector(vertex);
        }
        Self::new(self.center, vertices)
    }
}

impl PolyCollision for TriangleCollider {
    fn edges_of(&self) -> Vec<Vector2<f32>> {
        let vertices = self.vertices();
        let len = vertices.len();
        let mut edges: Vec<Vector2<f32>> = Vec::with_capacity(self.vertices.len());
        for i in 0..len {
            edges.push(vertices[(i + 1) % len] - vertices[i]);
        }
        edges
    }
    fn vertices(&self) -> Vec<Vector2<f32>> {
        self.vertices.iter().map(|v| self.center + v).collect()
    }
    fn collide(&self, other: &impl PolyCollision) -> bool {
        let (collided, _vec) = is_collision(self, other);
        collided
    }
}
