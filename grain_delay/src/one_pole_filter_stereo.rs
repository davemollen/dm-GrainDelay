use std::f32::consts::TAU;

pub struct OnePoleFilterStereo {
  t: f32,
  z: (f32, f32),
  prev_freq: f32,
  b1: f32,
}

impl OnePoleFilterStereo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      t: sample_rate.recip() * -TAU,
      z: (0., 0.),
      prev_freq: 0.,
      b1: 0.,
    }
  }

  pub fn process(&mut self, input: (f32, f32), freq: f32) -> (f32, f32) {
    if freq != self.prev_freq {
      self.b1 = (freq * self.t).exp();
      self.prev_freq = freq;
    }

    let a0 = 1.0 - self.b1;
    self.z = (
      input.0 * a0 + self.z.0 * self.b1,
      input.1 * a0 + self.z.1 * self.b1,
    );
    self.z
  }
}
