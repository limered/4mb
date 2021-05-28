use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowSetup, NumSamples};
use ggez::event::{self, EventHandler};

fn main() {
    let window_setup = WindowSetup {
        title: "Timmy Hunting".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("timmy_hunting", "Emil Wasilewski")
        .window_setup(window_setup)
		.build()
		.expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e)
    }
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
		    // ...
		}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);
        // Draw code here...
		graphics::present(ctx)
    }
}