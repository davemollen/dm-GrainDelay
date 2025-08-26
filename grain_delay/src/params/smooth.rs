use std::f32::consts::TAU;

pub trait Smoother {
  fn reset(&mut self, target: f32);

  fn set_target(&mut self, target: f32);

  fn get_target(&self) -> f32;

  fn next(&mut self) -> f32;
}

pub struct LinearSmooth {
  current: f32,
  target: f32,
  factor: f32,
  step_size: f32,
  steps: usize,
  step_counter: usize,
}

impl LinearSmooth {
  pub fn new(sample_rate: f32, frequency: f32) -> Self {
    Self {
      current: 0.,
      target: 0.,
      factor: sample_rate.recip() * frequency,
      step_size: 0.,
      steps: (sample_rate / frequency) as usize,
      step_counter: 0,
    }
  }
}

impl Smoother for LinearSmooth {
  fn reset(&mut self, target: f32) {
    self.current = target;
    self.step_counter = 0;
  }

  fn set_target(&mut self, target: f32) {
    if target != self.target {
      self.step_size = (target - self.current) * self.factor;
      self.target = target;
      self.step_counter = self.steps;
    }
  }

  fn get_target(&self) -> f32 {
    self.target
  }

  fn next(&mut self) -> f32 {
    if self.step_counter > 0 {
      self.current += self.step_size;
      self.step_counter -= 1;
    } else {
      self.current = self.target;
    }
    self.current
  }
}

pub struct ExponentialSmooth {
  current: f32,
  target: f32,
  b1: f32,
  is_active: bool,
}

impl ExponentialSmooth {
  pub fn new(sample_rate: f32, frequency: f32) -> Self {
    Self {
      current: 0.,
      target: 0.,
      b1: (-TAU * frequency * sample_rate.recip()).exp(),
      is_active: false,
    }
  }
}

impl Smoother for ExponentialSmooth {
  fn reset(&mut self, target: f32) {
    self.current = target;
    self.is_active = false;
  }

  fn set_target(&mut self, target: f32) {
    self.target = target;
    self.is_active = self.current != self.target;
  }

  fn get_target(&self) -> f32 {
    self.target
  }

  fn next(&mut self) -> f32 {
    if self.is_active {
      let a0 = 1.0 - self.b1;
      self.current = self.target * a0 + self.current * self.b1;
      if (self.current - self.target).abs() <= f32::EPSILON {
        self.current = self.target;
        self.is_active = false;
      }
    }
    self.current
  }
}

pub struct LogarithmicSmooth {
  current: f32,
  target: f32,
  factor: f32,
  is_active: bool,
}

impl LogarithmicSmooth {
  pub fn new(sample_rate: f32, factor: f32) -> Self {
    Self {
      current: 0.,
      target: 0.,
      factor: (sample_rate * factor).recip(),
      is_active: false,
    }
  }
}

impl Smoother for LogarithmicSmooth {
  fn reset(&mut self, target: f32) {
    self.current = target;
    self.is_active = false;
  }

  fn set_target(&mut self, target: f32) {
    self.target = target;
    self.is_active = self.current != self.target;
  }

  fn get_target(&self) -> f32 {
    self.target
  }

  fn next(&mut self) -> f32 {
    if self.is_active {
      let difference = self.target - self.current;
      let ad = 0.693147 * self.factor;
      self.current += difference * ad;
      if (self.current - self.target).abs() <= f32::EPSILON {
        self.current = self.target;
        self.is_active = false;
      }
    }
    self.current
  }
}

#[cfg(test)]
mod tests {
  use crate::params::{smooth::LinearSmooth, Smoother};

  #[test]
  fn should_smooth_linearly() {
    let mut linear_smooth = LinearSmooth::new(5., 1.);
    linear_smooth.set_target(1.);
    assert_eq!(linear_smooth.next(), 0.2);
    linear_smooth.set_target(1.);
    assert_eq!(linear_smooth.next(), 0.4);
    assert_eq!(linear_smooth.next(), 0.6);
    assert_eq!(linear_smooth.next(), 0.8);
    assert_eq!(linear_smooth.next(), 1.0);
    assert_eq!(linear_smooth.next(), 1.0);
    linear_smooth.set_target(0.);
    assert_eq!(linear_smooth.next(), 0.8);
    linear_smooth.set_target(0.);
    assert_eq!(linear_smooth.next(), 0.6);
    assert_eq!(linear_smooth.next(), 0.40000004);
    assert_eq!(linear_smooth.next(), 0.20000003);
    assert_eq!(linear_smooth.next(), 2.9802322e-8);
    assert_eq!(linear_smooth.next(), 0.0);
  }
}
