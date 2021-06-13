use ggez::conf::{NumSamples, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::Vector2;
use ggez::{graphics, Context, ContextBuilder, GameResult};

use crate::entities::player;

mod collision_system;
mod destroyable;
mod entities;
mod physic;
mod render_system;

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

struct MyGame {
    pub player: player::Player,
    pub element: destroyable::Destroyable,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        MyGame {
            player: player::Player::new(ctx),
            element: destroyable::Destroyable::new(Vector2::new(600.0, 200.0), ctx),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        self.player.update(dt, &ctx);
        // collision_system::collide(&mut self.player, &mut self.element);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.player.render(ctx).expect("Error during Player Render");
        self.element
            .render(ctx)
            .expect("Error during Element Render");

        graphics::present(ctx)
    }
}
