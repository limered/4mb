#[derive(PartialEq)]
enum PlayerState {
    Sliding,
    Boosting,
}

pub struct Player {
    ang_acc: f32,
    lin_acc: f32,
    state: PlayerState,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::Sliding,
            ang_acc: 0.0,
            lin_acc: 0.0,
        }
    }

    pub fn rotate(&mut self, value: f32) {
        self.ang_acc = value;
    }

    pub fn boost(&mut self, value: f32) {
        self.lin_acc = value;
        if value < 0.0 {
          self.state = PlayerState::Boosting;
        } else {
          self.state = PlayerState::Sliding;
        }
    }
}
