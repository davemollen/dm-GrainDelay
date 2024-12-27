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
  is_active: bool,
}

impl LinearSmooth {
  pub fn new(sample_rate: f32, frequency: f32) -> Self {
    Self {
      current: 0.,
      target: 0.,
      factor: sample_rate.recip() * frequency,
      step_size: 0.,
      is_active: false,
    }
  }
}

impl Smoother for LinearSmooth {
  fn reset(&mut self, target: f32) {
    self.current = target;
    self.is_active = false;
  }

  fn set_target(&mut self, target: f32) {
    self.target = target;
    self.is_active = self.current != self.target;
    if self.is_active {
      self.step_size = (target - self.current) * self.factor;
    }
  }

  fn get_target(&self) -> f32 {
    self.target
  }

  fn next(&mut self) -> f32 {
    if !self.is_active {
      return self.current;
    }
    if (self.current - self.target).abs() <= f32::EPSILON {
      self.current = self.target;
      self.is_active = false;
      return self.target;
    }
    self.current += self.step_size;
    return self.current;
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
    if !self.is_active {
      return self.current;
    }
    if (self.current - self.target).abs() <= f32::EPSILON {
      self.current = self.target;
      self.is_active = false;
      return self.target;
    }
    let a0 = 1.0 - self.b1;
    self.current = self.target * a0 + self.current * self.b1;
    return self.current;
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
    if !self.is_active {
      return self.current;
    }
    if (self.current - self.target).abs() <= f32::EPSILON {
      self.current = self.target;
      self.is_active = false;
      return self.target;
    }
    let difference = self.target - self.current;
    let ad = 0.693147 * self.factor;
    self.current += difference * ad;
    return self.current;
  }
}
