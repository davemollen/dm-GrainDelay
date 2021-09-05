pub struct Lowpass {
  z: f32,
}

impl Lowpass {
  pub fn new() -> Self {
    Self { z: 0. }
  }
  fn mix(&mut self, a: f32, b: f32, interp: f32) -> f32 {
    a * (1. - interp) + b * interp
  }
  pub fn run(&mut self, x: f32, coeff: f32) -> f32 {
    let y = self.mix(x, self.z, coeff);
    self.z = y;
    y
  }
}
