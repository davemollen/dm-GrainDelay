#[derive(Clone)]
pub struct Ramp {
  sample_rate: f32,
  x: Option<f32>,
  is_active: bool,
}

impl Ramp {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      x: None,
      is_active: false,
    }
  }

  fn initialize(&mut self, step_size: f32) -> f32 {
    let x = if step_size > 0. { 0. } else { 1. };
    self.x = Some(x);
    x
  }

  fn get_step_size(&self, freq: f32) -> f32 {
    1. / self.sample_rate * freq
  }

  fn keep_between_bounds(&mut self, next_x: f32) -> f32 {
    if next_x <= 0. || next_x >= 1. {
      self.is_active = false;
    }
    if next_x >= 0. && next_x <= 1. {
      self.x = Some(next_x);
      next_x
    } else {
      self.x = None;
      0.
    }
  }

  pub fn start(&mut self, starts_at: Option<f32>) {
    self.is_active = true;
    self.x = starts_at;
  }

  pub fn is_finished(&self) -> bool {
    !self.is_active
  }

  pub fn run(&mut self, freq: f32, has_bounds: bool) -> f32 {
    if self.is_active {
      let step_size = self.get_step_size(freq);

      match self.x {
        None => self.initialize(step_size),
        Some(current_x) => {
          let next_x = current_x + step_size;
          if has_bounds {
            self.keep_between_bounds(next_x)
          } else {
            self.x = Some(next_x);
            next_x
          }
        }
      }
    } else {
      0.
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Ramp;

  fn assert_approximately_eq(left: f32, right: f32) {
    assert_eq!(
      (left * 1000.).round() / 1000.,
      (right * 1000.).round() / 1000.
    )
  }

  #[test]
  fn forward_ramp() {
    let mut ramp = Ramp::new(10.);
    ramp.start(None);
    assert_approximately_eq(ramp.run(1., true), 0.);
    assert_approximately_eq(ramp.run(1., true), 0.1);
    assert_approximately_eq(ramp.run(1., true), 0.2);
  }

  #[test]
  fn backwards_ramp() {
    let mut ramp = Ramp::new(10.);
    ramp.start(None);
    assert_approximately_eq(ramp.run(-1., true), 1.);
    assert_approximately_eq(ramp.run(-1., true), 0.9);
    assert_approximately_eq(ramp.run(-1., true), 0.8);
  }

  #[test]
  fn resets_ramp() {
    let mut ramp = Ramp::new(10.);
    ramp.start(Some(0.5));
    assert_approximately_eq(ramp.x.unwrap(), 0.5);
    assert_approximately_eq(ramp.run(1., true), 0.6);
    ramp.start(Some(0.6));
    assert_approximately_eq(ramp.x.unwrap(), 0.6);
    assert_approximately_eq(ramp.run(1., true), 0.7);
  }

  #[test]
  fn sets_active() {
    let mut ramp = Ramp::new(5.);
    ramp.start(None);
    assert!(ramp.is_active);
    assert!(!ramp.is_finished());
    assert_approximately_eq(ramp.run(1., true), 0.);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., true), 0.2);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., true), 0.4);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., true), 0.6);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., true), 0.8);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., true), 1.);
    assert!(ramp.is_active == false);
    assert!(ramp.is_finished())
  }
}
