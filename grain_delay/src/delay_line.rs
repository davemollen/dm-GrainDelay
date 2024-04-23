use std::f32::consts::PI;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Interpolation {
  Step,
  Linear,
  Cosine,
  Cubic,
  Spline,
}

#[derive(Clone)]
pub struct DelayLine {
  buffer: Vec<f32>,
  write_pointer: usize,
  sample_rate: f32,
  wrap: usize,
}

impl DelayLine {
  pub fn new(length: usize, sample_rate: f32) -> Self {
    let size = length.next_power_of_two();
    Self {
      buffer: vec![0.0; size],
      write_pointer: 0,
      sample_rate,
      wrap: size - 1,
    }
  }

  pub fn read(&mut self, time: f32, interp: Interpolation) -> f32 {
    match interp {
      Interpolation::Step => self.step_interp(time),
      Interpolation::Linear => self.linear_interp(time),
      Interpolation::Cosine => self.cosine_interp(time),
      Interpolation::Cubic => self.cubic_interp(time),
      Interpolation::Spline => self.spline_interp(time),
    }
  }

  pub fn write(&mut self, value: f32) {
    self.buffer[self.write_pointer] = value;
    self.write_pointer = self.write_pointer + 1 & self.wrap;
  }

  fn step_interp(&self, time: f32) -> f32 {
    let read_pointer = (self.write_pointer + self.buffer.len()) as f32 - (self.mstosamps(time) - 0.5).max(1.);
    let index = read_pointer.trunc() as usize;

    self.buffer[index + 1 & self.wrap]
  }

  fn linear_interp(&self, time: f32) -> f32 {
    let read_pointer = (self.write_pointer + self.buffer.len()) as f32 - self.mstosamps(time).max(1.);
    let rounded_read_pointer = read_pointer.trunc();
    let mix = read_pointer - rounded_read_pointer;
    let index = rounded_read_pointer as usize;

    let x = self.buffer[index & self.wrap];
    let y = self.buffer[index + 1 & self.wrap];
    x * (1. - mix) + y * mix
  }

  fn cosine_interp(&self, time: f32) -> f32 {
    let read_pointer = (self.write_pointer + self.buffer.len()) as f32 - self.mstosamps(time).max(1.);
    let rounded_read_pointer = read_pointer.trunc();
    let mix = read_pointer - rounded_read_pointer;
    let index = rounded_read_pointer as usize;

    let cosine_mix = (1. - (mix * PI).cos()) / 2.;
    let x = self.buffer[index & self.wrap];
    let y = self.buffer[index + 1 & self.wrap];
    x * (1. - cosine_mix) + y * cosine_mix
  }

  fn cubic_interp(&self, time: f32) -> f32 {
    let read_pointer = (self.write_pointer + self.buffer.len()) as f32 - self.mstosamps(time).max(2.);
    let rounded_read_pointer = read_pointer.trunc();
    let mix = read_pointer - rounded_read_pointer;
    let index = rounded_read_pointer as usize;

    let w = self.buffer[index & self.wrap];
    let x = self.buffer[index + 1 & self.wrap];
    let y = self.buffer[index + 2 & self.wrap];
    let z = self.buffer[index + 3 & self.wrap];

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

  fn spline_interp(&self, time: f32) -> f32 {
    let read_pointer = (self.write_pointer + self.buffer.len()) as f32 - self.mstosamps(time).max(2.);
    let rounded_read_pointer = read_pointer.trunc();
    let mix = read_pointer - rounded_read_pointer;
    let index = rounded_read_pointer as usize;

    let w = self.buffer[index & self.wrap];
    let x = self.buffer[index + 1 & self.wrap];
    let y = self.buffer[index + 2 & self.wrap];
    let z = self.buffer[index + 3 & self.wrap];

    let c0 = x;
    let c1 = (0.5) * (y - w);
    let c2 = w - (2.5) * x + y + y - (0.5) * z;
    let c3 = (0.5) * (z - w) + (1.5) * (x - y);
    ((c3 * mix + c2) * mix + c1) * mix + c0
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate
  }
}