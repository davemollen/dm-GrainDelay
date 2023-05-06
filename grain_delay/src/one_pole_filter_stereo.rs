use crate::float_ext::FloatExt;
use std::f32::consts::PI;

#[allow(dead_code)]
pub enum Mode {
  Linear,
  Hertz,
}

pub struct OnePoleFilterStereo {
  sample_rate: f32,
  z: (f32, f32),
}

impl OnePoleFilterStereo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate: sample_rate as f32,
      z: (0., 0.),
    }
  }

  fn convert_linear_input_to_coefficient(&self, r: f32) -> f32 {
    (1. - r) / 44100. * self.sample_rate
  }

  fn convert_hertz_to_coefficient(&self, freq: f32) -> f32 {
    let coef = (freq * 2. * PI / self.sample_rate).fast_sin();
    coef.clamp(0., 1.)
  }

  fn mix(&self, a: (f32, f32), b: (f32, f32), interp: f32) -> (f32, f32) {
    (
      a.0 * (1. - interp) + b.0 * interp,
      a.1 * (1. - interp) + b.1 * interp,
    )
  }

  fn z_is_subnormal(&self, input: (f32, f32)) -> bool {
    (input.0 - self.z.0).abs().is_subnormal() && (input.1 - self.z.1).abs().is_subnormal()
  }

  fn apply_filter(&mut self, input: (f32, f32), cutoff_freq: f32, mode: Mode) -> (f32, f32) {
    let coefficient = match mode {
      Mode::Linear => self.convert_linear_input_to_coefficient(cutoff_freq),
      Mode::Hertz => self.convert_hertz_to_coefficient(cutoff_freq),
    };
    let output = self.mix(self.z, input, coefficient);
    self.z = output;
    output
  }

  pub fn run(&mut self, input: (f32, f32), cutoff_freq: f32, mode: Mode) -> (f32, f32) {
    if self.z_is_subnormal(input) {
      input
    } else {
      self.apply_filter(input, cutoff_freq, mode)
    }
  }
}
