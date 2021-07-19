use crate::constants::*;
use crate::systems::player_system::player::Player;
use crate::PhysicsSystem;
use core::f32::consts::PI;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::*;
use nalgebra::Vector2;

pub mod player;
pub mod player_mesh_creator;

pub struct PlayerSystem {
    player: Player,
}

impl PlayerSystem {
    pub fn new(physic_system: &mut PhysicsSystem) -> Self {
        PlayerSystem {
            player: Player::new(physic_system),
        }
    }

    pub fn process_inputs(&mut self) {
        if is_key_down(KeyCode::D) {
            self.player.rotate(PLAYER_TURN);
        } else if is_key_down(KeyCode::A) {
            self.player.rotate(-PLAYER_TURN);
        } else {
            self.player.rotate(0.0);
        }

        if is_key_down(KeyCode::W) {
            self.player.boost(-PLAYER_ACC);
        } else {
            self.player.boost(0.0);
        }
    }

    pub fn update(&mut self, physics: &mut PhysicsSystem, _game_time: f32, dt: f32) {
        let body = physics
            .rigid_body_set
            .get_mut(self.player.body_handle)
            .unwrap();
        let force = body.rotation() * Vector2::new(0.0, self.player.lin_acc);

        body.apply_torque(self.player.ang_acc, true);
        body.apply_force(force, true);
    }

    pub fn render(&mut self, physics: &mut PhysicsSystem) {
        let body = physics
            .rigid_body_set
            .get_mut(self.player.body_handle)
            .unwrap();

        let translation = body.position().translation;
        let rotation = body.position().rotation;

        draw_poly_lines(
            translation.x,
            translation.y,
            3,
            50.0,
            rotation.angle() * (180.0 / PI),
            2.0,
            BLUE,
        );
    }
}
