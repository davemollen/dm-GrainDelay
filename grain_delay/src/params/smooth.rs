use std::f32::consts::TAU;

pub trait Smoother {
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
  is_initialized: bool,
}

impl LinearSmooth {
  pub fn new(frequency: f32, sample_rate: f32) -> Self {
    Self {
      current: 0.,
      target: 0.,
      factor: sample_rate.recip() * frequency,
      step_size: 0.,
      is_active: false,
      is_initialized: false,
    }
  }
}

impl Smoother for LinearSmooth {
  fn set_target(&mut self, target: f32) {
    self.target = target;
    if self.is_initialized {
      let diff = target - self.current;
      self.step_size = diff * self.factor;
      self.is_active = diff.abs() > f32::EPSILON;
    } else {
      self.current = target;
      self.is_active = false;
      self.is_initialized = true;
    };
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
  is_initialized: bool,
}

impl ExponentialSmooth {
  pub fn new(frequency: f32, sample_rate: f32) -> Self {
    Self {
      current: 0.,
      target: 0.,
      b1: (-TAU * frequency * sample_rate.recip()).exp(),
      is_active: false,
      is_initialized: false,
    }
  }
}

impl Smoother for ExponentialSmooth {
  fn set_target(&mut self, target: f32) {
    self.target = target;
    if self.is_initialized {
      self.is_active = (self.current - self.target).abs() > f32::EPSILON;
    } else {
      self.current = target;
      self.is_active = false;
      self.is_initialized = true;
    };
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
  is_initialized: bool,
}

impl LogarithmicSmooth {
  pub fn new(factor: f32, sample_rate: f32) -> Self {
    Self {
      current: 0.,
      target: 0.,
      factor: (sample_rate * factor).recip(),
      is_active: false,
      is_initialized: false,
    }
  }
}

impl Smoother for LogarithmicSmooth {
  fn set_target(&mut self, target: f32) {
    self.target = target;
    if self.is_initialized {
      self.is_active = (self.current - self.target).abs() > f32::EPSILON;
    } else {
      self.current = target;
      self.is_active = false;
      self.is_initialized = true;
    };
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
