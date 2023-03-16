#[derive(Clone)]
pub struct Ramp {
  sample_rate: f32,
  x: Option<f32>,
  is_active: bool,
  started_in_reverse: bool,
  progress: f32,
}

impl Ramp {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      x: None,
      is_active: false,
      started_in_reverse: false,
      progress: 0.,
    }
  }

  fn initialize(&mut self, step_size: f32, min: f32, max: f32) -> f32 {
    let x = if step_size > 0. { min } else { max };
    self.started_in_reverse = step_size < 0.;
    self.x = Some(x);
    x
  }

  fn get_step_size(&self, freq: f32) -> f32 {
    1. / self.sample_rate * freq
  }

  fn set_progress(&mut self, x: f32, min: f32, max: f32) {
    let position = (x - min) * (max - min).recip();
    self.progress = if self.started_in_reverse {
      1. - position
    } else {
      position
    };
  }

  fn keep_between_bounds(&mut self, next_x: f32, min: f32, max: f32) -> f32 {
    if next_x <= min || next_x >= max {
      self.is_active = false;
    }
    let x = next_x.max(min).min(max);
    self.x = Some(x);
    x
  }

  pub fn get_progress(&self) -> f32 {
    self.progress
  }

  pub fn start(&mut self, starts_at: Option<f32>) {
    self.is_active = true;
    self.x = starts_at;
  }

  pub fn is_finished(&self) -> bool {
    !self.is_active
  }

  pub fn get_x(&mut self, freq: f32, min: f32, max: f32) -> f32 {
    if self.is_active {
      let step_size = self.get_step_size(freq);

      match self.x {
        None => self.initialize(step_size, min, max),
        Some(current_x) => self.keep_between_bounds(current_x + step_size, min, max),
      }
    } else {
      self.x.unwrap()
    }
  }

  pub fn run(&mut self, freq: f32, min: f32, max: f32) -> f32 {
    let x = self.get_x(freq, min, max);
    self.set_progress(x, min, max);
    x
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
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.1);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.2);
  }

  #[test]
  fn backwards_ramp() {
    let mut ramp = Ramp::new(10.);
    ramp.start(None);
    assert_approximately_eq(ramp.run(-1., 0., 1.), 1.);
    assert_approximately_eq(ramp.run(-1., 0., 1.), 0.9);
    assert_approximately_eq(ramp.run(-1., 0., 1.), 0.8);
  }

  #[test]
  fn resets_ramp() {
    let mut ramp = Ramp::new(10.);
    ramp.start(Some(0.5));
    assert_approximately_eq(ramp.x.unwrap(), 0.5);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.6);
    ramp.start(Some(0.6));
    assert_approximately_eq(ramp.x.unwrap(), 0.6);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.7);
  }

  #[test]
  fn sets_active() {
    let mut ramp = Ramp::new(5.);
    ramp.start(None);
    assert!(ramp.is_active);
    assert!(!ramp.is_finished());
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.2);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.4);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.6);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.8);
    assert!(ramp.is_active);
    assert_approximately_eq(ramp.run(1., 0., 1.), 1.);
    assert!(ramp.is_active == false);
    assert!(ramp.is_finished())
  }

  #[test]
  fn stays_between_bounds() {
    let mut ramp = Ramp::new(10.);
    ramp.start(Some(1.));
    assert_approximately_eq(ramp.run(1., 0., 1.2), 1.1);
    assert_approximately_eq(ramp.run(1., 0., 1.2), 1.2);
    assert!(ramp.is_active == false);
    assert!(ramp.is_finished());

    ramp.start(Some(0.));
    assert_approximately_eq(ramp.run(-1., -0.2, 1.), -0.1);
    assert_approximately_eq(ramp.run(-1., -0.2, 1.), -0.2);
    assert!(ramp.is_active == false);
    assert!(ramp.is_finished())
  }

  #[test]
  fn get_progress() {
    let mut ramp = Ramp::new(10.);
    ramp.start(None);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.);
    assert_approximately_eq(ramp.get_progress(), 0.);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.1);
    assert_approximately_eq(ramp.get_progress(), 0.1);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.2);
    assert_approximately_eq(ramp.get_progress(), 0.2);

    ramp.start(None);
    assert_approximately_eq(ramp.run(1., 0., 2.), 0.);
    assert_approximately_eq(ramp.get_progress(), 0.);
    assert_approximately_eq(ramp.run(1., 0., 2.), 0.1);
    assert_approximately_eq(ramp.get_progress(), 0.05);
    assert_approximately_eq(ramp.run(1., 0., 2.), 0.2);
    assert_approximately_eq(ramp.get_progress(), 0.1);

    ramp.start(None);
    assert_approximately_eq(ramp.run(-1., 0., 2.), 2.);
    assert_approximately_eq(ramp.get_progress(), 0.);
    assert_approximately_eq(ramp.run(-1., 0., 2.), 1.9);
    assert_approximately_eq(ramp.get_progress(), 0.05);
    assert_approximately_eq(ramp.run(-1., 0., 2.), 1.8);
    assert_approximately_eq(ramp.get_progress(), 0.1);

    ramp.start(None);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.);
    assert_approximately_eq(ramp.get_progress(), 0.);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.1);
    assert_approximately_eq(ramp.get_progress(), 0.1);
    assert_approximately_eq(ramp.run(1., 0., 1.), 0.2);
    assert_approximately_eq(ramp.get_progress(), 0.2);
    assert_approximately_eq(ramp.run(-1., 0., 1.), 0.1);
    assert_approximately_eq(ramp.get_progress(), 0.1);
    assert_approximately_eq(ramp.run(-1., 0., 1.), 0.);
    assert_approximately_eq(ramp.get_progress(), 0.);
    assert!(ramp.is_finished());
  }
}
