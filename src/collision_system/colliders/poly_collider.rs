use crate::collision_system::{PolyCollision, Rotatable};

use ggez::nalgebra as na;
use na::{Rotation2, Vector2};

pub struct PolyCollider {
    pub center: Vector2<f32>,
    pub vertices: Vec<Vector2<f32>>,
}

impl PolyCollider {
    pub fn new(center: Vector2<f32>, vertices: Vec<Vector2<f32>>) -> Self {
        PolyCollider { center, vertices }
    }
}

impl Rotatable for PolyCollider {
    fn rotate(&mut self, rotation: &Rotation2<f32>) {
        let mut vertices = [Vector2::x(); 4];
        for (i, vertex) in self.vertices.iter().enumerate() {
            vertices[i] = rotation.transform_vector(vertex);
        }
    }
}

impl PolyCollision for PolyCollider {
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
}
