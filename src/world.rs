use crate::systems::player_system::PlayerSystem;
use crate::PhysicsSystem;
use macroquad::prelude::*;

pub struct GameWorld {
    player_system: PlayerSystem,
    physics_system: PhysicsSystem,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut physics_system = PhysicsSystem::new();
        GameWorld {
            player_system: PlayerSystem::new(&mut physics_system),
            physics_system,
        }
    }
    pub fn process_inputs(&mut self) {
        self.player_system.process_inputs();
    }

    pub fn update(&mut self, game_time: f32, dt: f32) {
        self.player_system
            .update(&mut self.physics_system, game_time, dt);
    }

    pub fn fixed_update(&mut self, _game_time: f32, _dt: f32) {
        self.physics_system.update();
    }

    pub fn interpolate(&mut self, _alpha: f32) {
        // let state: State = curr * alpha + prev * (1.0 - alpha);
    }

    pub fn render(&mut self) {
        clear_background(BLACK);

        self.player_system.render(&mut self.physics_system);
    }
}
