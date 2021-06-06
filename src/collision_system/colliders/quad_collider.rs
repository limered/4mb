use crate::collision_system::{is_collision, PolyCollision, Rotatable};

use ggez::nalgebra as na;
use na::{Rotation2, Vector2};

pub struct QuadCollider {
    pub center: Vector2<f32>,
    pub vertices: [Vector2<f32>; 4],
}

impl QuadCollider {
    pub fn new(center: Vector2<f32>, vertices: [Vector2<f32>; 4]) -> Self {
        Self { center, vertices }
    }
}

impl Rotatable for QuadCollider {
    fn rotate(&self, rotation: Rotation2<f32>) -> Self {
        let mut vertices = [Vector2::x(); 4];
        for (i, vertex) in self.vertices.iter().enumerate() {
            vertices[i] = rotation.transform_vector(vertex);
        }
        Self::new(self.center, vertices)
    }
}

impl PolyCollision for QuadCollider {
    fn edges_of(&self) -> Vec<Vector2<f32>> {
        let mut edges: Vec<Vector2<f32>> = Vec::with_capacity(self.vertices.len());
        let len = self.vertices.len();
        for i in 0..len {
            let edge = self.vertices[(i + 1) % len] - self.vertices[i];
            edges.push(edge);
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
