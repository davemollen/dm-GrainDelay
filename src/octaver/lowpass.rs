use super::clip::Clip;
pub const PI: f64 = 3.14159265358979323846264338327950288f64;

pub struct Lowpass {
  sample_rate: f64,
  z: f32,
}

impl Lowpass {
  pub fn new(sample_rate: f64) -> Self {
    Self { sample_rate, z: 0. }
  }
  fn mix(&mut self, a: f32, b: f32, interp: f32) -> f32 {
    a * (1. - interp) + b * interp
  }
  pub fn run(&mut self, input: f32, freq: f32) -> f32 {
    let coef = f32::sin(freq * PI as f32 / (self.sample_rate as f32));
    let clipped_coef = Clip::run(coef, 0., 1.);
    let output = self.mix(self.z, input, clipped_coef);
    self.z = output;
    output
  }
}
