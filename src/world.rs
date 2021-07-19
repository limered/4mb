use crate::systems::player_system::PlayerSystem;
use macroquad::prelude::*;

pub struct GameWorld {
    player_system: PlayerSystem,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            player_system: PlayerSystem::new(),
        }
    }
    pub fn process_inputs(&mut self) {
        self.player_system.process_inputs();
    }
    pub fn update(&mut self, game_time: f32, dt: f32) {
        self.player_system.update(game_time, dt);
    }
    pub fn interpolate(&mut self, _alpha: f32) {
        // let state: State = curr * alpha + prev * (1.0 - alpha);
    }
    pub fn render(&mut self) {
        clear_background(BLACK);

        draw_poly_lines(40.0, 40.0, 6, 50.0, 0.0, 2.0, BLUE);
    }
}
