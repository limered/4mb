use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::Point2;
use ggez::Context;
use rapier2d::prelude::*;

pub fn create_player_mesh(ctx: &mut Context) -> Mesh {
    let main_reckt = Rect::new(-4.0, -20.0, 8.0, 30.0);
    let left_reckt = Rect::new(-12.0, 0.0, 8.0, 10.0);
    let right_reckt = Rect::new(4.0, 0.0, 8.0, 10.0);

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
            &[Point2::new(-3.0, 15.0), Point2::new(-6.0, 23.0)],
            2.0,
            WHITE,
        )
        .unwrap()
        .line(
            &[Point2::new(0.0, 15.0), Point2::new(0.0, 24.0)],
            2.0,
            WHITE,
        )
        .unwrap()
        .line(
            &[Point2::new(3.0, 15.0), Point2::new(6.0, 23.0)],
            2.0,
            WHITE,
        )
        .unwrap()
        .build(ctx)
        .expect("Could not create player boost mesh")
}

pub fn build_player_collider() -> Vec<Collider> {
    vec![
        ColliderBuilder::cuboid(4.0, 15.0)
            .density(0.0)
            .translation(Vector::new(0.0, 0.0))
            .build(),
        ColliderBuilder::cuboid(4.0, 5.0)
            .density(0.0)
            .translation(Vector::new(-4.0, 15.0))
            .build(),
        ColliderBuilder::cuboid(4.0, 5.0)
            .density(0.0)
            .translation(Vector::new(4.0, 15.0))
            .build(),
    ]
}
