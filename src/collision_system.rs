use na::{Isometry2, Vector2};
use nalgebra as na;
use ncollide2d::query::{self, DefaultTOIDispatcher, TOI};
use ncollide2d::shape::Polyline;

pub trait Collidable {
    fn process_overlap(&mut self, mpv: Vector2<f32>);
    fn process_collision(&mut self, other: &impl Collidable, n: &Vector2<f32>, t: f32);
    fn position(&self) -> Isometry2<f32>;
    fn collider(&self) -> Polyline<f32>;
    fn velocity(&self) -> Vector2<f32>;
}

pub fn collide(first: &mut impl Collidable, second: &mut impl Collidable) {
    let toi = query::time_of_impact(
        &DefaultTOIDispatcher,
        &first.position(),
        &first.velocity(),
        &first.collider(),
        &second.position(),
        &second.velocity(),
        &second.collider(),
        10.0,
        10.0,
    )
    .unwrap();

    match toi {
        Some(x) => {
            resolve_collision(x, first, second);
        }
        _ => {}
    }
}

fn resolve_collision(toi: TOI<f32>, first: &mut impl Collidable, second: &mut impl Collidable) {
    if toi.toi == 0.0 {
        //Resolve for Collision
        second.process_collision(first, &Vector2::zeros(), 0.0);
    } else {
        //Resolve for Impact
        second.process_overlap(Vector2::zeros());
    }
}
