use crate::systems::render_system::camera::Camera;
use crate::PhysicsSystem;
use ggez::graphics::Color;
use ggez::graphics::Mesh;
use nalgebra::Isometry2;
use nalgebra::Point2;
use nalgebra::Vector2;

pub mod camera;

pub trait Renderable {
    fn position(&self, physics: &PhysicsSystem) -> Vector2<f32>;
    fn rotation(&self, physics: &PhysicsSystem) -> f32;
    fn color(&self) -> ggez::graphics::Color;
    fn info_as_ref(&self) -> Vec<RenderInfo>;
}

#[derive(PartialEq, Clone)]
pub struct RenderInfo {
    pub is_visible: bool,
    pub is_player: bool,
    mesh: Mesh,
    position: Vector2<f32>,
    position_last: Vector2<f32>,
    rotation: f32,
    rotation_last: f32,
    scale: f32,
    color: ggez::graphics::Color,
}

impl RenderInfo {
    pub fn new(mesh: Mesh, color: ggez::graphics::Color) -> Self {
        RenderInfo {
            mesh,
            color,
            position: Vector2::zeros(),
            position_last: Vector2::zeros(),
            rotation: 0.0,
            rotation_last: 0.0,
            scale: 1.0,
            is_visible: true,
            is_player: false,
        }
    }

    pub fn update(&mut self, position: Vector2<f32>, rotation: f32, color: Color) {
        self.position_last = self.position;
        self.rotation_last = self.rotation;
        self.position = position;
        self.rotation = rotation;
        self.color = color;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}

pub struct RenderSystem {
    camera: Camera,
    renderables: Vec<RenderInfo>,
}

impl RenderSystem {
    pub fn new() -> Self {
        RenderSystem {
            camera: Camera::new(Isometry2::new(Vector2::new(0.0, 300.0), 0.0)),
            renderables: Vec::new(),
        }
    }

    pub fn add_to_render(&mut self, infos: Vec<RenderInfo>) {
        for info in infos.iter() {
            self.renderables.push(info.clone());
        }
    }
    pub fn render(&mut self, ctx: &mut ggez::Context) {
        for info in &self.renderables {
            if info.is_player {
                self.camera.set_transform_position(info.position);
            }
            if info.is_visible {
                let pos = self.camera.modify(info.position);
                ggez::graphics::draw(
                    ctx,
                    &info.mesh,
                    (
                        point_to_point(pos),
                        info.rotation,
                        ggez::nalgebra::Point2::new(0.0,0.0),
                        ggez::nalgebra::Vector2::new(info.scale, info.scale),
                        info.color,
                    ),
                )
                .unwrap();
            }
        }
        self.renderables.clear();
    }
}

fn point_to_point(p: Point2<f32>) -> ggez::nalgebra::Point2<f32> {
    ggez::nalgebra::Point2::new(p.x, p.y)
}
