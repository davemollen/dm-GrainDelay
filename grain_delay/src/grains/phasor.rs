pub struct Phasor {
  sample_period: f32,
  x: f32,
}

impl Phasor {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_period: sample_rate.recip(),
      x: 0.,
    }
  }

  pub fn process(&mut self, freq: f32) -> f32 {
    self.x = self.wrap(self.x + freq * self.sample_period);
    self.x
  }

  fn wrap(&self, input: f32) -> f32 {
    if input >= 1. {
      input - 1.
    } else {
      input
    }
  }
}
