use std::f32;

pub struct DelayLine {
  buffer: Vec<f32>,
  write_pointer: usize,
  sample_rate: f64,
}

impl DelayLine {
  pub fn new(length: usize, sample_rate: f64) -> Self {
    Self {
      buffer: vec![0.0; length],
      write_pointer: 0,
      sample_rate,
    }
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate as f32
  }

  fn wrap(&self, read_pointer: f32) -> usize {
    read_pointer as usize % self.buffer.len()
  }

  fn step_interp(&self, read_pointer: f32) -> f32 {
    let index = read_pointer.trunc();
    self.buffer[self.wrap(index)]
  }

  fn linear_interp(&self, read_pointer: f32) -> f32 {
    let index = read_pointer.trunc();
    let mix = read_pointer - index;
    let x = self.buffer[self.wrap(index)];
    let y = self.buffer[self.wrap(index + 1.)];
    x * (1. - mix) + y * mix
  }

  fn cosine_interp(&self, read_pointer: f32) -> f32 {
    let index = read_pointer.trunc();
    let mix = read_pointer - index;
    let cosine_mix = (1. - (mix * f32::consts::PI).cos()) / 2.;
    let x = self.buffer[self.wrap(index)];
    let y = self.buffer[self.wrap(index + 1.)];
    x * (1. - cosine_mix) + y * cosine_mix
  }

  fn cubic_interp(&self, read_pointer: f32) -> f32 {
    let index = read_pointer.trunc() - 1.;
    let mix = read_pointer - index;
    let w = self.buffer[self.wrap(index)];
    let x = self.buffer[self.wrap(index + 1.)];
    let y = self.buffer[self.wrap(index + 2.)];
    let z = self.buffer[self.wrap(index + 3.)];

    let a1 = 1. + mix;
    let aa = mix * a1;
    let b = 1. - mix;
    let b1 = 2. - mix;
    let bb = b * b1;
    let fw = -0.1666667 * bb * mix;
    let fx = 0.5 * bb * a1;
    let fy = 0.5 * aa * b1;
    let fz = -0.1666667 * aa * b;
    w * fw + x * fx + y * fy + z * fz
  }

  fn spline_interp(&self, read_pointer: f32) -> f32 {
    let index = read_pointer.trunc() - 1.;
    let mix = read_pointer - index;
    let w = self.buffer[self.wrap(index)];
    let x = self.buffer[self.wrap(index + 1.)];
    let y = self.buffer[self.wrap(index + 2.)];
    let z = self.buffer[self.wrap(index + 3.)];

    let c0 = x;
    let c1 = (0.5) * (y - w);
    let c2 = w - (2.5) * x + y + y - (0.5) * z;
    let c3 = (0.5) * (z - w) + (1.5) * (x - y);
    ((c3 * mix + c2) * mix + c1) * mix + c0
  }

  pub fn read(&mut self, time: f32, interp: &str) -> f32 {
    let read_pointer = self.write_pointer as f32 - self.mstosamps(time) + self.buffer.len() as f32;
    match interp {
      "step" => self.step_interp(read_pointer),
      "linear" => self.linear_interp(read_pointer),
      "cosine" => self.cosine_interp(read_pointer),
      "cubic" => self.cubic_interp(read_pointer),
      "spline" => self.spline_interp(read_pointer),
      _ => self.step_interp(read_pointer),
    }
  }

  pub fn write(&mut self, value: f32) {
    if self.write_pointer >= self.buffer.len() {
      self.write_pointer = 0;
    } else {
      self.write_pointer += 1;
    }
    self.buffer[self.write_pointer] = value;
  }
}
