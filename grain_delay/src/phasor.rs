pub struct Phasor {
  sample_rate: f32,
  x: f32,
}

impl Phasor {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, x: 0. }
  }

  fn wrap(&self, input: f32) -> f32 {
    if input >= 1. {
      input - 1.
    } else if input <= 0. {
      input + 1.
    } else {
      input
    }
  }

  pub fn process(&mut self, freq: f32) -> f32 {
    let multiplier = 1. / self.sample_rate;
    self.x = self.wrap(self.x + freq * multiplier);
    self.x
  }
}
