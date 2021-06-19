use ggez::graphics::{Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::Point2;
use ggez::Context;
use rapier2d::prelude::*;

pub fn create_player_mesh(player_points: &[(f32, f32); 3], ctx: &mut Context) -> Mesh {
    let mesh_points: [Point2<f32>; 3] = [
        Point2::new(player_points[0].0, player_points[0].1),
        Point2::new(player_points[1].0, player_points[1].1),
        Point2::new(player_points[2].0, player_points[2].1),
    ];
    MeshBuilder::new()
        .line(&mesh_points, 2.0, WHITE)
        .unwrap()
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
        ColliderBuilder::cuboid(5.0, 20.0).density(0.0).build(),
        ColliderBuilder::cuboid(5.0, 10.0)
            .density(0.0)
            .translation(Vector::new(-5.0, -5.0))
            .build(),
        ColliderBuilder::cuboid(5.0, 10.0)
            .density(0.0)
            .translation(Vector::new(5.0, -5.0))
            .build(),
    ]
}
