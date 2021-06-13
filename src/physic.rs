use ggez::nalgebra::{Point2, Vector2};

pub struct Body {
    pub position: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub drag: f32,
    pub velocity: Vector2<f32>,
}

pub struct Transform {
    pub position: Point2<f32>,
    pub rotation: f32,
}

impl Body {
    pub fn new(position: Vector2<f32>, drag: f32) -> Self {
        Body {
            position: position,
            drag: drag,
            acceleration: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
        }
    }
    pub fn animate(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;

        self.velocity *= self.drag;
    }

    pub fn stop(&mut self) {
        self.velocity = Vector2::new(0.0, 0.0);
    }
}

impl Transform {
    pub fn new(position: Point2<f32>, rotation: f32) -> Self {
        Transform {
            position: position,
            rotation: rotation,
        }
    }
}
