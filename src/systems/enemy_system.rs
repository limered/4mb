use crate::systems::enemy_system::scanline::Scanline;
use core::f32::consts::PI;
use ggez::Context;
use nalgebra::Vector2;
use rand::Rng;
use rapier2d::prelude::*;

use crate::constants::*;
use crate::player::Player;
use crate::systems::enemy_system::enemy::*;
use crate::systems::render_system::*;
use crate::PhysicsSystem;
use crate::RenderSystem;
use crate::Renderable;

pub struct EnemySystem {
    pub enemies: Vec<Enemy>,
    pub enemies_to_remove: Vec<usize>,
    pub enemies_to_spawn: Vec<(Vector2<f32>, EnemySize)>,
    scanline: Scanline,
}

impl EnemySystem {
    pub fn new(ctx: &mut Context) -> Self {
        EnemySystem {
            enemies: Vec::new(),
            enemies_to_remove: Vec::new(),
            enemies_to_spawn: Vec::new(),
            scanline: Scanline::new(ctx),
        }
    }

    fn calculate_spawn_position(angle: f32) -> Vector2<f32> {
        let mut rng = rand::thread_rng();
        let spawn_radius: f32 =
            (rng.gen::<f32>() * (SPAWN_RANGE.1 - SPAWN_RANGE.0)) + SPAWN_RANGE.0;
        let real_angle = angle + PI / 2.0;
        let x = spawn_radius * real_angle.cos();
        let y = spawn_radius * real_angle.sin();
        Vector2::new(x, y)
    }

    pub fn make_enemy(
        &mut self,
        ctx: &mut Context,
        physics: &mut PhysicsSystem,
        size: EnemySize,
        angle: f32,
    ) {
        let position = Self::calculate_spawn_position(angle) + Vector2::new(MIDDLE.0, MIDDLE.1);
        self.make_enemy_at_position(ctx, physics, position, size);
    }

    pub fn make_enemy_at_position(
        &mut self,
        ctx: &mut Context,
        physics: &mut PhysicsSystem,
        position: Vector2<f32>,
        size: EnemySize,
    ) {
        let mut enemy = Enemy::new(ctx, position, physics, size);
        enemy.render_info.render_effect = RenderEffect::HitShift(15.0, 1.0);
        enemy.render_info.render_effect_t = 0.0;
        self.enemies.push(enemy);
    }

    pub fn update(&mut self, dt: f32, ctx: &mut Context, physics: &mut PhysicsSystem) {
        let (spawn, angle) = self.scanline.update(dt, physics);
        if spawn {
            self.make_enemy(ctx, physics, EnemySize::Big, angle);
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
            if enemy.life_time < 0.0 {
                self.enemies_to_remove.push(i);
            }
        }
    }

    pub fn process_collisions(
        &mut self,
        physics: &mut PhysicsSystem,
        player: &Player,
        earth_handle: &ColliderHandle,
    ) {
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            if enemy.size == EnemySize::Small {
                continue;
            }

            let mut valid_collision = false;

            for collider_pair in physics.narrow_phase.contacts_with(enemy.collider_handle) {
                let other_collider = if collider_pair.collider1 == enemy.collider_handle {
                    collider_pair.collider2
                } else {
                    collider_pair.collider1
                };

                if other_collider == player.collider_handles[0]
                    || other_collider == player.collider_handles[1]
                {
                    valid_collision = true;
                    enemy.deletion_reason = DeletionReason::Player;
                }
                if other_collider == player.collider_handles[2] {
                    valid_collision = false;
                }
                if other_collider == *earth_handle {
                    valid_collision = true;
                    enemy.deletion_reason = DeletionReason::Earth;
                }
            }

            if valid_collision {
                self.enemies_to_remove.push(i);
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
            if enemy.deletion_reason == DeletionReason::Player {
                match enemy.size {
                    EnemySize::Big => self
                        .enemies_to_spawn
                        .push((*body.translation(), EnemySize::Middle)),
                    EnemySize::Middle => self
                        .enemies_to_spawn
                        .push((*body.translation(), EnemySize::Small)),
                    EnemySize::Small => {}
                };
            } else if enemy.deletion_reason == DeletionReason::Earth {
                // Do Stuff for earth collision
            }
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
        render.add_to_render(self.scanline.info_as_ref());
    }
}

pub mod enemy;
pub mod scanline;
