use crate::systems::render_system::RenderSystem;
use crate::systems::render_system::Renderable;
use ggez::conf::{NumSamples, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

use crate::entities::player;
use crate::entities::world::{BoundedByWorld, World};
use crate::systems::enemy_system::EnemySystem;
use crate::systems::physic_system::PhysicsSystem;

mod constants;
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
    pub render_system: RenderSystem,
    accumulator: f32,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let render_system = RenderSystem::new();
        let mut physic_system = PhysicsSystem::new();
        let mut game = MyGame {
            player: Option::None,
            enemy_system: EnemySystem::new(ctx),
            world: World::new(ctx, &mut physic_system),
            physic_system,
            render_system,
            accumulator: 0.0,
        };
        let player = player::Player::new(ctx, &mut game);
        game.player = Option::Some(player);
        game
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = systems::physic_system::DT;
        let mut frame_time = ggez::timer::delta(ctx).as_secs_f32();
        if frame_time > 0.1 {
            frame_time = 0.1;
        }

        self.accumulator += frame_time;

        while self.accumulator >= dt {
            if let Some(player) = &mut self.player {
                player.update(&ctx, &mut self.physic_system);
                self.world
                    .update(player.body_handle(), &mut self.physic_system, dt);
            }

            self.enemy_system.update(dt, ctx, &mut self.physic_system);

            self.physic_system.update();
            self.accumulator -= dt;

            if let Some(player) = &self.player {
                self.enemy_system.process_collisions(
                    &mut self.physic_system,
                    &player,
                    &self.world.earth_collider_handle,
                );
            }
        }

        self.enemy_system
            .process_enemies_to_remove(&mut self.physic_system);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if let Some(player) = &self.player {
            self.render_system.add_to_render(player.info_as_ref());
            self.render_system.add_to_render(player.boost.info_as_ref());
        }
        self.render_system.add_to_render(self.world.info_as_ref());
        self.enemy_system
            .add_to_render_system(&mut self.render_system);

        self.render_system.render(ctx);
        ggez::timer::sleep(std::time::Duration::from_secs(0));
        graphics::present(ctx)
    }
}
