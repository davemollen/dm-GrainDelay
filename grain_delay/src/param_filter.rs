use std::f32::consts::TAU;

use crate::float_ext::FloatExt;

pub struct ParamFilter {
  b1: f32,
  z: f32,
}

impl ParamFilter {
  pub fn new(sample_rate: f32, filter_freq: f32) -> Self {
    Self {
      b1: (-TAU * filter_freq * sample_rate.recip()).exp(),
      z: 0.,
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    if input.is_equal_to(self.z) {
      input
    } else {
      let a0 = 1.0 - self.b1;
      self.z = input * a0 + self.z * self.b1;
      self.z
    }
  }
}
