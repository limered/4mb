use crate::constants::COLL_PLAYER_HEAVY;
use crate::constants::COLL_PLAYER;
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::Point2;
use ggez::Context;
use rapier2d::prelude::*;

pub fn create_player_mesh(ctx: &mut Context) -> Mesh {
    let main_reckt = Rect::new(-5.0, -16.0, 10.0, 32.0);
    let left_reckt = Rect::new(-13.0, 3.0, 8.0, 13.0);
    let right_reckt = Rect::new(5.0, 3.0, 8.0, 13.0);

    MeshBuilder::new()
        .rectangle(DrawMode::stroke(2.0), main_reckt, WHITE)
        .rectangle(DrawMode::fill(), left_reckt, WHITE)
        .rectangle(DrawMode::fill(), right_reckt, WHITE)
        .build(ctx)
        .expect("Could not build PLayer Mesh")
}
pub fn create_player_boost_mesh(ctx: &mut Context) -> Mesh {
    MeshBuilder::new()
        .line(
            &[Point2::new(-3.0, 20.0), Point2::new(-6.0, 33.0)],
            2.0,
            WHITE,
        )
        .unwrap()
        .line(
            &[Point2::new(0.0, 20.0), Point2::new(0.0, 33.0)],
            2.0,
            WHITE,
        )
        .unwrap()
        .line(
            &[Point2::new(3.0, 20.0), Point2::new(6.0, 33.0)],
            2.0,
            WHITE,
        )
        .unwrap()
        .build(ctx)
        .expect("Could not create player boost mesh")
}

pub fn build_player_collider() -> Vec<Collider> {
    vec![
        ColliderBuilder::cuboid(5.0, 16.0).density(0.05).user_data(COLL_PLAYER).build(),
        ColliderBuilder::ball(13.0)
            .density(0.06)
            .translation(Vector::new(-10.0, 8.0))
            .user_data(COLL_PLAYER_HEAVY)
            .build(),
        ColliderBuilder::ball(13.0)
            .density(0.06)
            .translation(Vector::new(10.0, 8.0))
            .user_data(COLL_PLAYER_HEAVY)
            .build(),
    ]
}
