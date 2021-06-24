use crate::systems::render_system::camera::Camera;
use crate::PhysicsSystem;
use ggez::graphics::Color;
use ggez::graphics::Mesh;
use ggez::graphics::{set_blend_mode, BlendMode};
use nalgebra::Isometry2;
use nalgebra::Point2;
use nalgebra::Vector2;
use rand::Rng;

pub mod camera;

pub trait Renderable {
    fn position(&self, physics: &PhysicsSystem) -> Vector2<f32>;
    fn rotation(&self, physics: &PhysicsSystem) -> f32;
    fn color(&self) -> ggez::graphics::Color;
    fn info_as_ref(&self) -> Vec<RenderInfo>;
}

#[derive(PartialEq, Clone)]
pub enum RenderEffect {
    None,
    SpeedShift(f32),
    HitShift(f32, f32),
}

#[derive(PartialEq, Clone)]
pub struct RenderInfo {
    pub is_visible: bool,
    pub is_player: bool,
    pub velocity: Vector2<f32>,
    pub render_effect: RenderEffect,
    pub render_effect_t: f32,
    mesh: Mesh,
    position: Vector2<f32>,
    position_last: Vector2<f32>,
    rotation: f32,
    rotation_last: f32,
    scale: f32,
    color: ggez::graphics::Color,
    depth: i8,
}

impl RenderInfo {
    pub fn new(mesh: Mesh, color: ggez::graphics::Color, depth: i8) -> Self {
        RenderInfo {
            mesh,
            color,
            position: Vector2::zeros(),
            position_last: Vector2::zeros(),
            rotation: 0.0,
            rotation_last: 0.0,
            scale: 1.0,
            velocity: Vector2::zeros(),
            is_visible: true,
            is_player: false,
            depth,
            render_effect: RenderEffect::None,
            render_effect_t: 0.0,
        }
    }

    pub fn depth(&mut self, depth: i8) {
        self.depth = depth;
    }

    pub fn update(&mut self, position: Vector2<f32>, rotation: f32, color: Color, dt: f32) {
        self.position_last = self.position;
        self.rotation_last = self.rotation;
        self.position = position;
        self.rotation = rotation;
        self.color = color;
        if self.render_effect != RenderEffect::None {
            self.render_effect_t += dt;
        }
        match self.render_effect {
            RenderEffect::HitShift(_, d) => {
                if self.render_effect_t > d {
                    self.render_effect = RenderEffect::None;
                }
            }
            _ => {}
        };
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
        // println!("{}",self.renderables.len());
        self.renderables.sort_by(|a, b| b.depth.cmp(&a.depth));
        self.camera.update();

        set_blend_mode(ctx, BlendMode::Add).expect("");

        for info in &self.renderables {
            RenderSystem::update_camera(&mut self.camera, info);
            RenderSystem::render_info(ctx, &self.camera, info);
        }
        self.renderables.clear();
    }

    fn render_info(ctx: &mut ggez::Context, camera: &Camera, info: &RenderInfo) {
        if info.is_visible {
            let positions = RenderSystem::split_positions(info);
            let colors = RenderSystem::split_colors(info);
            RenderSystem::render_effect(ctx, camera, info, positions.0, colors.0);
            RenderSystem::render_effect(ctx, camera, info, positions.1, colors.1);
            RenderSystem::render_effect(ctx, camera, info, positions.2, colors.2);
        }
    }

    fn split_positions(info: &RenderInfo) -> (Vector2<f32>, Vector2<f32>, Vector2<f32>) {
        match info.render_effect {
            RenderEffect::SpeedShift(strength) => {
                let distance = info.position_last - info.position;
                let middle = distance * strength * 0.5;
                (
                    info.position + middle,
                    info.position,
                    info.position - middle,
                )
            }
            RenderEffect::HitShift(strength, duration) => {
                let t = info.render_effect_t / duration;
                let s = 1.0 - (1.0 - (t - 1.0).powf(2.0)).sqrt();
                let s = s * strength;
                (
                    info.position + random_vector(s),
                    info.position + random_vector(s),
                    info.position + random_vector(s),
                )
            }
            _ => (info.position, info.position, info.position),
        }
    }

    fn split_colors(
        info: &RenderInfo,
    ) -> (
        ggez::graphics::Color,
        ggez::graphics::Color,
        ggez::graphics::Color,
    ) {
        let color = info.color;
        (
            ggez::graphics::Color::new(color.r, 0.0, 0.0, 1.0),
            ggez::graphics::Color::new(0.0, color.g, 0.0, 1.0),
            ggez::graphics::Color::new(0.0, 0.0, color.b, 1.0),
        )
    }

    fn update_camera(camera: &mut Camera, info: &RenderInfo) {
        if info.is_player {
            camera.set_transform_position(info.position, info.position_last, info.rotation);
        }
    }

    fn render_effect(
        ctx: &mut ggez::Context,
        camera: &Camera,
        info: &RenderInfo,
        pos: Vector2<f32>,
        color: ggez::graphics::Color,
    ) {
        let pos = camera.modify(pos);
        ggez::graphics::draw(
            ctx,
            &info.mesh,
            (
                point_to_point(pos),
                info.rotation,
                ggez::nalgebra::Point2::new(0.0, 0.0),
                ggez::nalgebra::Vector2::new(info.scale, info.scale),
                color,
            ),
        )
        .unwrap();
    }
}

fn point_to_point(p: Point2<f32>) -> ggez::nalgebra::Point2<f32> {
    ggez::nalgebra::Point2::new(p.x, p.y)
}

fn sin_split(t: f32, strength: f32) -> Vector2<f32> {
    Vector2::new(0.0, 0.0)
}

fn random_vector(strength: f32) -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    let x_rng: f32 = rng.gen();
    let x_rng = (x_rng * strength) - strength / 2.0;
    let y_rng: f32 = rng.gen();
    let y_rng = (y_rng * strength) - strength / 2.0;
    Vector2::new(x_rng, y_rng)
}
