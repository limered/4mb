use crate::constants::*;
use crate::systems::player_system::player::Player;
use macroquad::input::{is_key_down, KeyCode};

pub mod player;

pub struct PlayerSystem {
    player: Player,
}

impl PlayerSystem {
    pub fn new() -> Self {
        PlayerSystem {
            player: Player::new(),
        }
    }

    pub fn process_inputs(&mut self) {
        if is_key_down(KeyCode::D) {
            self.player.rotate(PLAYER_TURN);
        } else if is_key_down(KeyCode::A) {
            self.player.rotate(-PLAYER_TURN);
        }

        if is_key_down(KeyCode::W) {
            self.player.boost(-PLAYER_ACC);
        } else {
            self.player.boost(0.0);
        }
    }

    pub fn update(&mut self, _game_time: f32, _dt: f32) {}
}
