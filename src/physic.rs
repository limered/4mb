use ggez::nalgebra::Vector2;

pub struct Physics {
    pub position: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub drag: f32,
    velocity: Vector2<f32>,
}

impl Physics {
    pub fn new(position: Vector2<f32>, drag: f32) -> Self {
        Physics {
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
        self.acceleration = Vector2::new(0.0, 0.0);
        self.velocity = Vector2::new(0.0, 0.0);
    }
}
