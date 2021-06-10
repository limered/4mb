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
    fn respond(&mut self, mpv: Vector2<f32>);
    fn collider(&self) -> &PolyCollider;
}

pub fn collide(first: &mut impl Collidable, second: &mut impl Collidable) {
    let mut coll = sat::Coll::new(
        first.collider(),
        second.collider(),
        &Vector2::new(0.0, 0.0),
        &Vector2::new(0.0, 0.0),
    );
    if coll.collide() {
        println!("Hit {}", coll.t)
    }
    // if coll.collide() {
    //     first.respond(coll.n);
    //     second.respond(coll.n);
    // }
}
