use core::f32::consts::PI;
use ggez::Context;
use nalgebra::Vector2;
use rand::Rng;
use rapier2d::prelude::*;

use crate::systems::enemy_system::enemy::Enemy;
use crate::PhysicsSystem;
use crate::RenderSystem;
use crate::Renderable;

const SPAWN_RANGE: (f32, f32) = (300.0, 500.0);
const SPAWN_TIME: f32 = 1.0;
const MIDDLE: (f32, f32) = (400.0, 300.0);

pub struct EnemySystem {
    pub enemies: Vec<Enemy>,
    pub spawn_timer: f32,
}

impl EnemySystem {
    pub fn new() -> Self {
        EnemySystem {
            enemies: Vec::new(),
            spawn_timer: 0.0,
        }
    }

    fn calculate_spawn_position() -> Vector2<f32> {
        let mut rng = rand::thread_rng();
        let spawn_radius: f32 =
            (rng.gen::<f32>() * (SPAWN_RANGE.1 - SPAWN_RANGE.0)) + SPAWN_RANGE.0;
        let spawn_angle: f32 = rng.gen::<f32>() * 2.0 * PI;
        let x = spawn_radius * spawn_angle.cos();
        let y = spawn_radius * spawn_angle.sin();
        Vector2::new(x, y)
    }

    pub fn make_enemy(&mut self, ctx: &mut Context, physics: &mut PhysicsSystem) {
        let position = Self::calculate_spawn_position() + Vector2::new(MIDDLE.0, MIDDLE.1);
        let enemy = Enemy::new(ctx, position, physics);
        self.enemies.push(enemy);
    }

    pub fn update(&mut self, dt: f32, ctx: &mut Context, physics: &mut PhysicsSystem) {
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 {
            self.make_enemy(ctx, physics);
            self.spawn_timer = SPAWN_TIME;
        }

        for enemy in self.enemies.iter_mut() {
            enemy.update(dt, ctx, physics);
        }
    }

    pub fn add_to_render_system(&mut self, render: &mut RenderSystem) {
        for enemy in &self.enemies {
            render.add_to_render(enemy.info_as_ref());
        }
    }
}

pub mod enemy;
