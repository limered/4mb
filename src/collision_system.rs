use crate::collision_system::colliders::poly_collider::PolyCollider;
use ggez::nalgebra as na;
use na::{Rotation2, Vector2};

pub mod colliders;
pub mod sat;

pub trait Rotatable {
    fn rotate(&mut self, rotation: &Rotation2<f32>);
}

pub trait PolyCollision {
    fn edges_of(&self) -> Vec<Vector2<f32>>;
    fn vertices(&self) -> Vec<Vector2<f32>>;
}

pub trait Collidable {
    fn collide(&mut self, other: &mut impl Collidable) -> (bool, Vector2<f32>);
    fn respond(&mut self, mpv: Vector2<f32>);
    fn collider(&self) -> &PolyCollider;
}

pub fn collide(first: &mut impl Collidable, second: &mut impl Collidable) {
    let (has_collision, mpv) = first.collide(second);
    if has_collision {
        first.respond(mpv);
        second.respond(mpv);
    }
}
