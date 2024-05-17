pub struct PulseTrain {
  sample_period: f32,
  x: f32,
}

impl PulseTrain {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_period: sample_rate.recip(),
      x: 0.,
    }
  }

  fn wrap(&self, input: f32) -> (bool, f32) {
    if input >= 1. {
      (true, input - 1.)
    } else {
      (false, input)
    }
  }

  pub fn process(&mut self, freq: f32) -> bool {
    let (trigger, x) = self.wrap(self.x + freq * self.sample_period);
    self.x = x;
    trigger
  }
}
