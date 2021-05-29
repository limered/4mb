use ggez::conf::{NumSamples, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::{self as na, Point2, Vector2};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::input::keyboard::{self, KeyCode};

fn main() {
    let window_setup = WindowSetup {
        title: "Timmy Hunting".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("timmy_hunting", "Emil Wasilewski")
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
    pub rotation: f32,
    pub direction: na::Rotation2<f32>, 
    pub position: Point2<f32>,
    pub triangle_mesh: graphics::Mesh,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        MyGame {
            rotation: 0.0,
            direction: na::Rotation2::new(0.0),
            position: Point2::new(300.0, 400.0),
            triangle_mesh: graphics::MeshBuilder::new()
                .line(
                    &[
                        Point2::new(15.0, 15.0),
                        Point2::new(0.0, -15.0),
                        Point2::new(-15.0, 15.0),
                    ],
                    2.0,
                    graphics::WHITE,
                )
                .unwrap()
                .build(ctx)
                .unwrap(),
        }
    }
}

fn move_player(position: &mut Point2<f32>, direction: &na::Rotation2<f32>, dt: f32, ctx: &Context) -> GameResult<()>{
    let mut movement: f32 = 0.0;
    if keyboard::is_key_pressed(ctx, KeyCode::W){
        movement = -100.0;
    }else if keyboard::is_key_pressed(ctx, KeyCode::S){
        movement = 100.0;
    }
    let delta = direction * Vector2::new(0.0, movement) * dt;
    position.y += delta.y;
    position.x += delta.x;

    Ok(())
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        move_player(&mut self.position, &self.direction, dt, &ctx)?;

        let mut r = na::Rotation2::new(0.0);
        if keyboard::is_key_pressed(ctx, KeyCode::D){
            r = na::Rotation2::new(1.0 * dt);
        }else if keyboard::is_key_pressed(ctx, KeyCode::A){
            r = na::Rotation2::new(-1.0 * dt);
        }
        self.direction = r * self.direction;

        self.rotation = self.direction.angle();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        
        graphics::draw(
            ctx,
            &self.triangle_mesh,
            (self.position, self.rotation, graphics::WHITE),
        )?;

        graphics::present(ctx)
    }
}
