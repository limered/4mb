use ggez::nalgebra as na;
use na::Vector2;

use crate::collision_system::PolyCollision;

pub fn is_collision(me: &impl PolyCollision, other: &impl PolyCollision) -> (bool, Vector2<f32>) {
    let mut edges = me.edges_of();
    edges.extend(other.edges_of().iter());
    let mut orthogonals: Vec<Vector2<f32>> = Vec::with_capacity(edges.len());
    for (_i, v) in edges.iter().enumerate() {
        orthogonals.push(otho(v));
    }
    let mut push_vectors: Vec<Vector2<f32>> = Vec::new();
    for o in &orthogonals {
        let (separated, pv) = is_separating_axis(o, me, other);
        if separated {
            return (false, Vector2::new(0.0, 0.0));
        } else {
            push_vectors.push(pv);
        }
    }

    let mpv = min_vec(&push_vectors);
    (true, mpv)
}

fn is_separating_axis(
    o: &Vector2<f32>,
    p1: &impl PolyCollision,
    p2: &impl PolyCollision,
) -> (bool, Vector2<f32>) {
    let p1_vertices = p1.vertices();
    let p2_vertices = p2.vertices();

    let mut min1 = f32::MAX;
    let mut min2 = f32::MAX;
    let mut max1 = -f32::MAX;
    let mut max2 = -f32::MAX;

    for v in &p1_vertices {
        let projection = v.dot(o);

        min1 = min1.min(projection);
        max1 = max1.max(projection);
    }

    for v in &p2_vertices {
        let projection = v.dot(o);

        min2 = min2.min(projection);
        max2 = max2.max(projection);
    }

    if max1 >= min2 && max2 >= min1 {
        let d = (max2 - min1).min(max1 - min2);
        let d_over_o_squared = d / o.dot(o) + 1e-10;
        let pv = d_over_o_squared * o;
        return (false, pv);
    }
    (true, Vector2::new(0.0, 0.0))
}

fn otho(v: &Vector2<f32>) -> Vector2<f32> {
    Vector2::new(-v.y, v.x)
}

fn min_vec(vec: &Vec<Vector2<f32>>) -> Vector2<f32> {
    let mut smallest = Vector2::new(f32::MAX, f32::MAX);
    let mut smallest_len = f32::MAX;
    for v in vec {
        let len = v.norm();
        if len < smallest_len {
            smallest_len = len;
            smallest = v.clone();
        }
    }
    smallest
}
