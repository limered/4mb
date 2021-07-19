use crate::constants::D_ANIMATED;
use crate::systems::player_system::player_mesh_creator;
use crate::systems::render_system::RenderEffect;
use ggez::graphics::WHITE;
use ggez::Context;
use nalgebra::Vector2;
use rapier2d::prelude::*;

use crate::systems::render_system::{RenderInfo, Renderable};
use crate::PhysicsSystem;

#[derive(PartialEq)]
pub struct Boost {
    render_info: RenderInfo,
    player_body_handle: RigidBodyHandle,
    is_visible: bool,
}

impl Boost {
    pub fn new(ctx: &mut Context, player_body_handle: RigidBodyHandle) -> Self {
        let mut render_info = RenderInfo::new(
            player_mesh_creator::create_player_boost_mesh(ctx),
            WHITE,
            D_ANIMATED,
        );
        render_info.render_effect = RenderEffect::HitShift(5.0, 5.0);
        Boost {
            render_info,
            player_body_handle,
            is_visible: false,
        }
    }

    pub fn update(&mut self, physics: &PhysicsSystem) {
        self.render_info.update(
            self.position(physics),
            self.rotation(physics),
            self.color(),
            0.0,
        );
    }

    pub fn set_visibility(&mut self, is_visible: bool) {
        self.render_info.is_visible = is_visible;
    }
}

impl Renderable for Boost {
    fn position(&self, physics: &PhysicsSystem) -> Vector2<f32> {
        let body = physics.rigid_body_set.get(self.player_body_handle).unwrap();
        Vector2::new(body.translation().x, body.translation().y)
    }
    fn rotation(&self, physics: &PhysicsSystem) -> f32 {
        let body = physics.rigid_body_set.get(self.player_body_handle).unwrap();
        body.rotation().angle()
    }
    fn color(&self) -> ggez::graphics::Color {
        WHITE
    }
    fn info_as_ref(&self) -> Vec<RenderInfo> {
        vec![self.render_info.clone()]
    }
}
