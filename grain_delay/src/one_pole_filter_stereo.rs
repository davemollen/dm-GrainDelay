use crate::float_ext::FloatExt;
use std::f32::consts::PI;

#[allow(dead_code)]
pub enum Mode {
  Linear,
  Hertz,
}

pub struct OnePoleFilterStereo {
  sample_rate: f32,
  z_left: f32,
  z_right: f32,
}

impl OnePoleFilterStereo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate: sample_rate as f32,
      z_left: 0.,
      z_right: 0.,
    }
  }

  fn convert_linear_input_to_coefficient(&self, r: f32) -> f32 {
    (1. - r) / 44100. * self.sample_rate
  }

  fn convert_hertz_to_coefficient(&self, freq: f32) -> f32 {
    let coef = (freq * 2. * PI / self.sample_rate).fast_sin();
    coef.clamp(0., 1.)
  }

  fn mix(&self, a: f32, b: f32, interp: f32) -> f32 {
    a * (1. - interp) + b * interp
  }

  fn z_is_subnormal(&self, input: (f32, f32)) -> bool {
    (input.0 - self.z_left).abs().is_subnormal() && (input.1 - self.z_right).abs().is_subnormal()
  }

  fn apply_filter(
    &mut self,
    (in_left, in_right): (f32, f32),
    cutoff_freq: f32,
    mode: Mode,
  ) -> (f32, f32) {
    let coefficient = match mode {
      Mode::Linear => self.convert_linear_input_to_coefficient(cutoff_freq),
      Mode::Hertz => self.convert_hertz_to_coefficient(cutoff_freq),
    };
    let out_left = self.mix(self.z_left, in_left, coefficient);
    let out_right = self.mix(self.z_right, in_right, coefficient);
    self.z_left = out_left;
    self.z_right = out_right;
    (out_left, out_right)
  }

  pub fn run(&mut self, input: (f32, f32), cutoff_freq: f32, mode: Mode) -> (f32, f32) {
    if self.z_is_subnormal(input) {
      input
    } else {
      self.apply_filter(input, cutoff_freq, mode)
    }
  }
}
