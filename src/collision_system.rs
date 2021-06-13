use na::{Isometry2, Vector2};
use nalgebra as na;
use ncollide2d::query::{self, DefaultTOIDispatcher};
use ncollide2d::shape::Polyline;

pub trait Collidable {
    fn process_overlap(&mut self, mpv: Vector2<f32>);
    fn process_collision(&mut self, other: &impl Collidable, n: &Vector2<f32>, t: f32);
    fn position(&self) -> Isometry2<f32>;
    fn collider(&self) -> Polyline<f32>;
    fn velocity(&self) -> Vector2<f32>;
}

pub fn _collide(first: &mut impl Collidable, second: &mut impl Collidable) {
    let _toi = query::time_of_impact(
        &DefaultTOIDispatcher,
        &first.position(),
        &first.velocity(),
        &first.collider(),
        &second.position(),
        &second.velocity(),
        &second.collider(),
        100.0,
        100.0,
    );
}
