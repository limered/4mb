use ggez::conf::{NumSamples, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

use crate::entities::player;
use crate::entities::world::{BoundedByWorld, World};
use crate::systems::enemy_system::EnemySystem;
use crate::systems::physic_system::PhysicsSystem;

mod entities;
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
    pub physic_system: PhysicsSystem,
    pub enemy_system: EnemySystem,
    pub world: World,
    accumulator: f32,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let mut game = MyGame {
            player: Option::None,
            physic_system: PhysicsSystem::new(),
            enemy_system: EnemySystem::new(),
            world: World::new(ctx),
            accumulator: 0.0,
        };
        game.player = Option::Some(player::Player::new(ctx, &mut game));
        game
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = systems::physic_system::DT;
        let mut frame_time = ggez::timer::delta(ctx).as_secs_f32();
        if frame_time > 0.25 {
            frame_time = 0.25;
        }

        self.accumulator += frame_time;

        while self.accumulator >= dt {
            if let Some(player) = &mut self.player {
                player.update(&ctx, &mut self.physic_system);
                self.world
                    .update(player.body_handle(), &mut self.physic_system);
            }

            self.enemy_system.update(dt, ctx, &mut self.physic_system);

            self.physic_system.update();
            self.accumulator -= dt;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.world.render(ctx);

        if let Some(player) = &mut self.player {
            player
                .render(ctx, &mut self.physic_system)
                .expect("Error during Player Render");
        }

        self.enemy_system.render(ctx, &mut self.physic_system);

        ggez::timer::sleep(std::time::Duration::from_secs(0));
        graphics::present(ctx)
    }
}
