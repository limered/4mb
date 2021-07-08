use macroquad::prelude::*;

pub struct GameWorld {

}

impl GameWorld {
  pub fn new() -> Self {
    GameWorld{}
  }
  pub fn process_inputs(&mut self){}
  pub fn update(&mut self, _game_time:f32, _dt:f32) {}
  pub fn interpolate(&mut self, _alpha:f32){
    // let state: State = curr * alpha + prev * (1.0 - alpha);
  }
  pub fn render(&mut self){
    clear_background(BLACK);

    draw_poly_lines(40.0, 40.0, 6, 50.0, 0.0, 2.0, BLUE);
  }
}