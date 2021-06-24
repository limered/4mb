use ggez::graphics::Rect;
use crate::constants::*;
use crate::systems::render_system::*;
use crate::PhysicsSystem;
use crate::World;
use ggez::graphics::BlendMode;
use ggez::graphics::DrawMode;
use ggez::graphics::MeshBuilder;
use ggez::graphics::BLACK;
use ggez::nalgebra::Point2;
use ggez::Context;
use nalgebra::Vector2;

pub struct Health {
    pub circle_render_info: RenderInfo,
    pub quad_render_info: RenderInfo,
}

impl Health {
    pub fn new(ctx: &mut Context) -> Self {
        let circle_color = ggez::graphics::Color::new(0.7, 0.0, 0.0, 1.0);
        let quad_color = ggez::graphics::Color::new(0.7, 1.0, 0.0, 1.0);
        let mut circle_render_info = RenderInfo::new(
            MeshBuilder::new()
                .rectangle(
                    DrawMode::fill(),
                    Rect::new(-100.0, -100.0, 200.0, 200.0),
                    circle_color,
                )
                .build(ctx)
                .expect("Earth Died"),
                circle_color,
            _D_NEBULA,
        );
        circle_render_info.blend_mode = BlendMode::Alpha;
        let mut quad_render_info = RenderInfo::new(
            MeshBuilder::new()
                .circle(
                    DrawMode::fill(),
                    Point2::new(0.0, 0.0),
                    EARTH_RADIUS - 2.0,
                    1.0,
                    quad_color,
                )
                .build(ctx)
                .expect("Earth Died"),
                quad_color,
            _D_NEBULA+1,
        );
        quad_render_info.blend_mode = BlendMode::Add;
        Health {
            circle_render_info,
            quad_render_info,
        }
    }

    pub fn update(&mut self, world: &World) {
        let current_health = world.health as f32 / HEALTH as f32;
        let pos_y = MIDDLE.1 - 200.0 * (1.0 - current_health);
        self.quad_render_info.update(
            Vector2::new(MIDDLE.0, MIDDLE.1),
            0.0,
            self.color(),
            0.0,
        );

        self.circle_render_info.update(
            Vector2::new(MIDDLE.0, pos_y),
            0.0,
            self.color(),
            0.0,
        );
    }
}

impl Renderable for Health {
    fn position(&self, _: &PhysicsSystem) -> Vector2<f32> {
        Vector2::new(0.0, 0.0)
    }
    fn color(&self) -> ggez::graphics::Color {
        ggez::graphics::Color::new(0.7, 0.0, 0.0, 1.0)
    }
    fn rotation(&self, _: &PhysicsSystem) -> f32 {
        0.0
    }
    fn info_as_ref(&self) -> Vec<RenderInfo> {
        vec![
            self.circle_render_info.clone(),
            self.quad_render_info.clone(),
        ]
    }
}
