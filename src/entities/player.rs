use ggez::graphics::WHITE;
use nalgebra::Vector2;
use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::prelude::*;

use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;
use nalgebra as na;

use crate::constants::*;
use crate::entities::boost::Boost;
use crate::entities::player_mesh_creator;
use crate::entities::player_mesh_creator::build_player_collider;
use crate::entities::world::BoundedByWorld;
use crate::systems::render_system::*;
use crate::MyGame;
use crate::PhysicsSystem;

#[derive(PartialEq)]
enum PlayerState {
    Sliding,
    Boosting,
}

pub struct Player {
    pub boost: Boost,
    pub collider_handles: Vec<ColliderHandle>,
    render_info: RenderInfo,
    state: PlayerState,
    body_handle: RigidBodyHandle,
}

impl Player {
    pub fn new(ctx: &mut Context, game: &mut MyGame) -> Self {
        let rb = RigidBodyBuilder::new_dynamic()
            .translation(na::Vector2::new(300.0, 400.0))
            .linear_damping(0.5)
            .angular_damping(10.0)
            .can_sleep(false)
            .ccd_enabled(true)
            .build();
        let body_handle = game.physic_system.rigid_body_set.insert(rb);
        let mut collider_handles: Vec<ColliderHandle> = Vec::new();
        let mut colliders = build_player_collider();
        for _i in 0..colliders.len() {
            collider_handles.push(game.physic_system.collider_set.insert_with_parent(
                colliders.pop().unwrap(),
                body_handle,
                &mut game.physic_system.rigid_body_set,
            ));
        }
        let mut render_info = RenderInfo::new(
            player_mesh_creator::create_player_mesh(ctx),
            WHITE,
            D_ANIMATED,
        );
        render_info.render_effect = RenderEffect::SpeedShift(0.6);
        Player {
            state: PlayerState::Sliding,
            render_info,
            boost: Boost::new(ctx, body_handle),
            body_handle,
            collider_handles: collider_handles,
        }
    }

    pub fn update(&mut self, ctx: &Context, physics: &mut PhysicsSystem) {
        self.control_movement(ctx, physics);
        self.control_rotation(ctx, physics);
        self.render_info.update(
            self.position(physics),
            self.rotation(physics),
            self.color(),
            0.0,
        );
        self.render_info.is_player = true;
        self.boost.update(physics);
        self.boost
            .set_visibility(self.state == PlayerState::Boosting);
    }

    fn control_rotation(&mut self, ctx: &Context, physics: &mut PhysicsSystem) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let mut r = 0.0;
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            r = PLAYER_TURN;
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            r = -PLAYER_TURN;
        }
        let body = physics.rigid_body_set.get_mut(self.body_handle).unwrap();
        body.apply_torque(r * dt * 60.0, true);
    }

    fn control_movement(&mut self, ctx: &Context, physics: &mut PhysicsSystem) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let mut movement: f32 = 0.0;
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            movement = -PLAYER_ACC;
            self.state = PlayerState::Boosting;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            movement = PLAYER_ACC;
            self.state = PlayerState::Boosting;
        } else {
            self.state = PlayerState::Sliding;
        }

        let body = physics.rigid_body_set.get_mut(self.body_handle).unwrap();
        let force = body.rotation() * vector![0.0, movement];
        body.apply_force(force * dt * 60.0, true);
    }
}

impl Renderable for Player {
    fn position(&self, physics: &PhysicsSystem) -> Vector2<f32> {
        let body = physics.rigid_body_set.get(self.body_handle).unwrap();
        body.translation().clone()
    }
    fn rotation(&self, physics: &PhysicsSystem) -> f32 {
        let body = physics.rigid_body_set.get(self.body_handle).unwrap();
        body.rotation().angle()
    }
    fn color(&self) -> ggez::graphics::Color {
        ggez::graphics::WHITE
    }
    fn info_as_ref(&self) -> Vec<RenderInfo> {
        vec![self.render_info.clone()]
    }
}

impl BoundedByWorld for Player {
    fn body_handle(&self) -> &RigidBodyHandle {
        &self.body_handle
    }
}
