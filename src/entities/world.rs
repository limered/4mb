use ggez::graphics::DrawMode;
use ggez::graphics::{MeshBuilder, WHITE};
use ggez::nalgebra::Point2;
use ggez::Context;
use nalgebra::Vector2;
use rapier2d::prelude::*;

use crate::constants::*;
use crate::systems::render_system::*;
use crate::PhysicsSystem;

#[derive(PartialEq)]
pub struct World {
    pub earth_collider_handle: ColliderHandle,
    pub health: i32,
    render_info: RenderInfo,
    earth_render_info: RenderInfo,
    earth_body_handle: RigidBodyHandle,
    damage_cooldown: f32,
}

impl World {
    pub fn new(ctx: &mut Context, physics: &mut PhysicsSystem) -> Self {
        let earth_body = RigidBodyBuilder::new_static()
            .translation(Vector::new(MIDDLE.0, MIDDLE.1))
            .dominance_group(10)
            .build();
        let earth_collider = ColliderBuilder::ball(EARTH_RADIUS - 5.0)
            .restitution(1.0)
            .build();
        let earth_body_handle = physics.rigid_body_set.insert(earth_body);
        World {
            earth_body_handle,
            earth_collider_handle: physics.collider_set.insert_with_parent(
                earth_collider,
                earth_body_handle,
                &mut physics.rigid_body_set,
            ),
            render_info: RenderInfo::new(
                MeshBuilder::new()
                    .circle(
                        DrawMode::stroke(1.0),
                        Point2::new(MIDDLE.0, MIDDLE.1),
                        EXTERIOR_RADIUS,
                        1.0,
                        WHITE,
                    )
                    .build(ctx)
                    .expect("World generation failed."),
                WHITE,
                D_EARTH,
            ),
            earth_render_info: RenderInfo::new(
                MeshBuilder::new()
                    .circle(
                        DrawMode::stroke(1.0),
                        Point2::new(MIDDLE.0, MIDDLE.1),
                        EARTH_RADIUS,
                        5.0,
                        WHITE,
                    )
                    .build(ctx)
                    .expect("Earth Died"),
                WHITE,
                D_EARTH,
            ),
            health: HEALTH,
            damage_cooldown: DAMAGE_COOLDOWN,
        }
    }

    pub fn update(&mut self, handle: &RigidBodyHandle, physics: &mut PhysicsSystem, dt: f32) {
        let body = physics.rigid_body_set.get_mut(*handle).unwrap();
        let direction = Vector::new(MIDDLE.0, MIDDLE.1) - body.translation();
        if direction.norm() > EXTERIOR_RADIUS {
            let direction = direction.normalize();
            body.apply_force(direction * EXTERIOR_PULL_FORCE, true);
        }

        self.damage_cooldown -= dt;

        self.render_info.update(
            self.position(physics),
            self.rotation(physics),
            self.color(),
            dt,
        );

        self.earth_render_info
            .update(self.position(physics), self.rotation(physics), WHITE, dt);
    }

    pub fn add_damage_from_enemy(&mut self, data: u128) -> (bool, RigidBodyHandle){
        if self.damage_cooldown > 0.0 {
            return (false, self.earth_body_handle);
        }
        let damage = match data {
            COLL_PLAYER => DAMAGE_SMALL,
            COLL_PLAYER_HEAVY => DAMAGE_PLAYER,
            COLL_SMALL => DAMAGE_SMALL,
            COLL_MIDDLE => DAMAGE_MIDDLE,
            COLL_BIG => DAMAGE_BIG,
            _ => DAMAGE_SMALL,
        };

        if damage > 0 {
            self.damage_cooldown = DAMAGE_COOLDOWN;
            self.health -= damage;

            self.earth_render_info.render_effect = RenderEffect::HitShift(15.0, 2.0);
            self.earth_render_info.render_effect_t = 0.0;

            self.render_info.render_effect = RenderEffect::Wobble(30.0, 1.0);
            self.render_info.render_effect_t = 0.0;
        };

        if self.health <= 0 {
            self.earth_render_info.is_visible = false;
        }

        (self.health <= 0, self.earth_body_handle)
    }
}

pub trait BoundedByWorld {
    fn body_handle(&self) -> &RigidBodyHandle;
}

impl Renderable for World {
    fn position(&self, _: &PhysicsSystem) -> Vector2<f32> {
        Vector2::new(0.0, 0.0)
    }
    fn color(&self) -> ggez::graphics::Color {
        ggez::graphics::Color::new(0.2, 0.2, 0.3, 1.0)
    }
    fn rotation(&self, _: &PhysicsSystem) -> f32 {
        0.0
    }
    fn info_as_ref(&self) -> Vec<RenderInfo> {
        vec![self.render_info.clone(), self.earth_render_info.clone()]
    }
}
