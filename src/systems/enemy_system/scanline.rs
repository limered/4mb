use ggez::graphics::Color;
use ggez::graphics::MeshBuilder;
use ggez::graphics::WHITE;

use crate::systems::enemy_system::*;
use crate::systems::render_system::*;
use crate::PhysicsSystem;

pub struct Scanline {
    direction: f32,
    render_info: RenderInfo,
    current_time_slice: f32,
    spawn_timer: f32,
    color: Color,
}

impl Scanline {
    pub fn new(ctx: &mut Context) -> Self {
        Scanline {
            direction: 0.0,
            current_time_slice: 0.0,
            spawn_timer: SPAWN_TIME,
            color: WHITE,
            render_info: RenderInfo::new(
                MeshBuilder::new()
                    .line(
                        &[
                            ggez::nalgebra::Point2::new(0.0, EARTH_RADIUS),
                            ggez::nalgebra::Point2::new(0.0, SPAWN_RANGE.1),
                        ],
                        1.0,
                        WHITE,
                    )
                    .unwrap()
                    .build(ctx)
                    .unwrap(),
                WHITE,
                D_SCANLINE,
            ),
        }
    }

    pub fn update(&mut self, dt: f32, physics: &PhysicsSystem) -> (bool, f32) {
        self.spawn_timer -= dt;
        let mut spawn = false;
        if self.spawn_timer < 0.0 {
            self.spawn_timer = SPAWN_TIME;
            spawn = true;
        }
        // update rotation
        self.current_time_slice += dt;
        self.current_time_slice = self.current_time_slice % SCANLINE_FULL_ROTATION_TIME;
        self.direction = -2.0 * PI * self.current_time_slice / SCANLINE_FULL_ROTATION_TIME;

        let x = 1.0 - self.spawn_timer / SPAWN_TIME;
        let color_intensity = (1.0 - (x - 1.0).powf(2.0)).sqrt();
        self.color = Color::new(1.0, color_intensity, color_intensity, 1.0);

        self.render_info
            .update(self.position(physics), self.rotation(physics), self.color());

        (spawn, self.direction)
    }
}

impl Renderable for Scanline {
    fn info_as_ref(&self) -> std::vec::Vec<RenderInfo> {
        vec![self.render_info.clone()]
    }
    fn color(&self) -> ggez::graphics::Color {
        self.color
    }
    fn rotation(&self, _: &PhysicsSystem) -> f32 {
        self.direction
    }
    fn position(&self, _: &PhysicsSystem) -> Vector2<f32> {
        Vector2::new(MIDDLE.0, MIDDLE.1)
    }
}
