pub struct Delta {
  z: f32,
}

impl Delta {
  pub fn new() -> Delta {
    Delta { z: 0. }
  }
  pub fn run(&mut self, x: f32) -> f32 {
    let y = x - self.z;
    self.z = x;
    y
  }
}
