use core::f32::consts::PI;
use ggez::Context;
use nalgebra::Vector2;
use rand::Rng;
use rapier2d::prelude::*;

use crate::player::Player;
use crate::systems::enemy_system::enemy::Enemy;
use crate::systems::enemy_system::enemy::EnemySize;
use crate::PhysicsSystem;
use crate::RenderSystem;
use crate::Renderable;

const SPAWN_RANGE: (f32, f32) = (300.0, 500.0);
const SPAWN_TIME: f32 = 1.0;
const MIDDLE: (f32, f32) = (400.0, 300.0);

pub struct EnemySystem {
    pub enemies: Vec<Enemy>,
    pub enemies_to_remove: Vec<usize>,
    pub enemies_to_spawn: Vec<(Vector2<f32>, EnemySize)>,
    pub spawn_timer: f32,
}

impl EnemySystem {
    pub fn new() -> Self {
        EnemySystem {
            enemies: Vec::new(),
            enemies_to_remove: Vec::new(),
            enemies_to_spawn: Vec::new(),
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

    pub fn make_enemy(&mut self, ctx: &mut Context, physics: &mut PhysicsSystem, size: EnemySize) {
        let position = Self::calculate_spawn_position() + Vector2::new(MIDDLE.0, MIDDLE.1);
        self.make_enemy_at_position(ctx, physics, position, size);
    }

    pub fn make_enemy_at_position(
        &mut self,
        ctx: &mut Context,
        physics: &mut PhysicsSystem,
        position: Vector2<f32>,
        size: EnemySize,
    ) {
        let enemy = Enemy::new(ctx, position, physics, size);
        self.enemies.push(enemy);
    }

    pub fn update(&mut self, dt: f32, ctx: &mut Context, physics: &mut PhysicsSystem) {
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 {
            self.make_enemy(ctx, physics, EnemySize::Big);
            self.spawn_timer = SPAWN_TIME;
        }

        // Spawn smaller enemies
        for i in 0..self.enemies_to_spawn.len() {
            let data = self.enemies_to_spawn[i];
            let displacement = Vector2::new(-data.0.y, data.0.x).normalize() * 10.0;
            self.make_enemy_at_position(ctx, physics, data.0 + displacement, data.1);
            self.make_enemy_at_position(ctx, physics, data.0 - displacement, data.1);
        }
        self.enemies_to_spawn.clear();

        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            enemy.update(dt, ctx, physics);
            if enemy.size == EnemySize::Small && enemy.life_time < 0.0 {
                self.enemies_to_remove.push(i);
            }
        }
    }

    pub fn process_collisions(&mut self, physics: &mut PhysicsSystem, player: &Player) {
        for (i, enemy) in self.enemies.iter().enumerate() {
            if enemy.size == EnemySize::Small {
                continue;
            }
            for contact_pair in physics.narrow_phase.contacts_with(enemy.collider_handle) {
                let other_collider = if contact_pair.collider1 == enemy.collider_handle {
                    contact_pair.collider2
                } else {
                    contact_pair.collider1
                };
                if other_collider == player.collider_handles[1]
                    || other_collider == player.collider_handles[2]
                {
                    self.enemies_to_remove.push(i);
                }
            }
        }
    }

    pub fn process_enemies_to_remove(&mut self, physics: &mut PhysicsSystem) {
        for i in (0..self.enemies_to_remove.len()).rev() {
            let item = self.enemies_to_remove[i];
            if item >= self.enemies.len() {
                continue;
            }
            let enemy = self.enemies.remove(item);
            let body = physics.rigid_body_set.get(enemy.body_handle).unwrap();
            match enemy.size {
                EnemySize::Big => self
                    .enemies_to_spawn
                    .push((*body.translation(), EnemySize::Middle)),
                EnemySize::Middle => self
                    .enemies_to_spawn
                    .push((*body.translation(), EnemySize::Small)),
                EnemySize::Small => {}
            };
            physics.rigid_body_set.remove(
                enemy.body_handle,
                &mut physics.island_manager,
                &mut physics.collider_set,
                &mut physics.joint_set,
            );
        }
        self.enemies_to_remove.clear();
    }

    pub fn add_to_render_system(&mut self, render: &mut RenderSystem) {
        for enemy in &self.enemies {
            render.add_to_render(enemy.info_as_ref());
        }
    }
}

pub mod enemy;
