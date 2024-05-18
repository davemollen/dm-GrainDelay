use crate::shared::float_ext::FloatExt;
use std::f32::consts::TAU;

pub struct OnePoleFilterStereo {
  t: f32,
  z: (f32, f32),
}

impl OnePoleFilterStereo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      t: sample_rate.recip() * -TAU,
      z: (0., 0.),
    }
  }

  pub fn process(&mut self, input: (f32, f32), cutoff_freq: f32) -> (f32, f32) {
    let b1 = (cutoff_freq * self.t).fast_exp();
    let a0 = 1.0 - b1;
    self.z = (input.0 * a0 + self.z.0 * b1, input.1 * a0 + self.z.1 * b1);
    self.z
  }
}
