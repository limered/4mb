use crate::collision_system::{PolyCollision, Rotatable};

use ggez::nalgebra as na;
use na::{Rotation2, Vector2};

pub struct QuadCollider {
    vertices: [Vector2<f32>; 4],
}

impl QuadCollider {
    pub fn new(vertices: [Vector2<f32>; 4]) -> Self {
        Self { vertices }
    }
}

impl Rotatable for QuadCollider {
    fn rotate(&self, rotation: Rotation2<f32>) -> Self {
        let mut vertices = [Vector2::x(); 4];
        for (i, vertex) in self.vertices.iter().enumerate() {
            vertices[i] = rotation.transform_vector(vertex);
        }
        Self::new(vertices)
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
        self.vertices.to_vec()
    }
}
