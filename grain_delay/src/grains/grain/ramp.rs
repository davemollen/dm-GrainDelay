#[derive(Clone)]
pub struct Ramp {
  x: f32,
  trigger: bool,
  is_active: bool,
  sample_period: f32,
}

impl Ramp {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      x: 0.,
      trigger: false,
      is_active: false,
      sample_period: sample_rate.recip(),
    }
  }

  pub fn start(&mut self) {
    self.trigger = true;
    self.is_active = true;
  }

  pub fn process(&mut self, freq: f32) -> f32 {
    let step_size = self.sample_period * freq;

    if self.trigger {
      if freq >= 0. {
        self.x = 0.;
      } else {
        self.x = 1.;
      }
      self.trigger = false;
    } else if self.is_active {
      self.x += step_size;
      if freq > 0. && self.x >= 1. {
        self.x = 1.;
        self.is_active = false;
      }
      if freq < 0. && self.x <= 0. {
        self.x = 0.;
        self.is_active = false;
      }
    }

    self.x
  }
}

#[cfg(test)]
mod tests {
  use super::Ramp;

  fn assert_approximately_eq(left: f32, right: f32) {
    assert_eq!((left * 10.).round() / 10., (right * 10.).round() / 10.)
  }

  #[test]
  fn forward_ramp() {
    let mut ramp = Ramp::new(10.);
    ramp.start();
    assert_approximately_eq(ramp.process(1.), 0.);
    assert_approximately_eq(ramp.process(1.), 0.1);
    assert_approximately_eq(ramp.process(1.), 0.2);
    assert_approximately_eq(ramp.process(1.), 0.3);
    assert_approximately_eq(ramp.process(1.), 0.4);
    assert_approximately_eq(ramp.process(1.), 0.5);
    assert_approximately_eq(ramp.process(1.), 0.6);
    assert_approximately_eq(ramp.process(1.), 0.7);
    assert_approximately_eq(ramp.process(1.), 0.8);
    assert_approximately_eq(ramp.process(1.), 0.9);
    assert_approximately_eq(ramp.process(1.), 1.);
    assert_approximately_eq(ramp.process(1.), 1.);
  }

  #[test]
  fn backward_ramp() {
    let mut ramp = Ramp::new(10.);
    ramp.start();
    assert_approximately_eq(ramp.process(-1.), 1.);
    assert_approximately_eq(ramp.process(-1.), 0.9);
    assert_approximately_eq(ramp.process(-1.), 0.8);
    assert_approximately_eq(ramp.process(-1.), 0.7);
    assert_approximately_eq(ramp.process(-1.), 0.6);
    assert_approximately_eq(ramp.process(-1.), 0.5);
    assert_approximately_eq(ramp.process(-1.), 0.4);
    assert_approximately_eq(ramp.process(-1.), 0.3);
    assert_approximately_eq(ramp.process(-1.), 0.2);
    assert_approximately_eq(ramp.process(-1.), 0.1);
    assert_approximately_eq(ramp.process(-1.), 0.);
    assert_approximately_eq(ramp.process(-1.), 0.);
  }

  #[test]
  fn bidirectional_ramp() {
    let mut ramp = Ramp::new(10.);
    ramp.start();
    assert_approximately_eq(ramp.process(-1.), 1.);
    assert_approximately_eq(ramp.process(-1.), 0.9);
    assert_approximately_eq(ramp.process(-1.), 0.8);
    assert_approximately_eq(ramp.process(1.), 0.9);
    assert_approximately_eq(ramp.process(1.), 1.);
    assert_approximately_eq(ramp.process(1.), 1.);
    assert_approximately_eq(ramp.process(-1.), 0.9);
    ramp.start();
    assert_approximately_eq(ramp.process(1.), 0.);
  }
}
