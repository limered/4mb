use ggez::conf::{NumSamples, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use nalgebra::Vector2;

use crate::entities::player;
use crate::systems::physic_system::PhysicsSystem;

mod destroyable;
mod entities;
mod render_system;
mod systems;

fn main() {
    let window_setup = WindowSetup {
        title: "Trashinator".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("trashinator", "Emil Wasilewski")
        .window_setup(window_setup)
        .build()
        .expect("aieee, could not create ggez context!");

    let mut my_game = MyGame::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
}

pub struct MyGame {
    pub player: Option<player::Player>,
    pub element: destroyable::Destroyable,
    pub physic_system: PhysicsSystem,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let mut game = MyGame {
            player: Option::None,
            element: destroyable::Destroyable::new(Vector2::new(600.0, 200.0), ctx),
            physic_system: PhysicsSystem::new(),
        };
        game.player = Option::Some(player::Player::new(ctx, &mut game));
        game
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        if let Some(player) = &mut self.player {
            player.update(dt, &ctx, &mut self.physic_system);
        }

        self.physic_system.update();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if let Some(player) = &mut self.player {
            player
                .render(ctx, &mut self.physic_system)
                .expect("Error during Player Render");
        }

        self.element
            .render(ctx, &mut self.physic_system)
            .expect("Error during Collidable render");

        graphics::present(ctx)
    }
}
