use std::f32::consts::PI;

#[allow(dead_code)]
pub enum Mode {
  Linear,
  Hertz,
}

pub struct OnePoleFilter {
  sample_rate: f32,
  z: f32,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, z: 0. }
  }

  fn convert_linear_input_to_coefficient(&mut self, r: f32) -> f32 {
    (1. - r) / 44100. * self.sample_rate
  }

  fn convert_hertz_to_coefficient(&mut self, freq: f32) -> f32 {
    let coef = (freq * 2. * PI / self.sample_rate).sin();
    coef.clamp(0., 1.)
  }

  fn mix(&mut self, a: f32, b: f32, interp: f32) -> f32 {
    a * (1. - interp) + b * interp
  }

  fn apply_filter(&mut self, input: f32, cutoff_freq: f32, mode: Mode) -> f32 {
    let coefficient = match mode {
      Mode::Linear => self.convert_linear_input_to_coefficient(cutoff_freq),
      Mode::Hertz => self.convert_hertz_to_coefficient(cutoff_freq),
    };
    let output = self.mix(self.z, input, coefficient);
    self.z = output;
    output
  }

  pub fn run(&mut self, input: f32, cutoff_freq: f32, mode: Mode) -> f32 {
    if (input - self.z).abs().is_subnormal() {
      input
    } else {
      self.apply_filter(input, cutoff_freq, mode)
    }
  }
}
