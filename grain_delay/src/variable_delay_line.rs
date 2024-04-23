use crate::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
  ramp::Ramp,
};
use std::f32::consts::FRAC_PI_2;

pub struct VariableDelayLine {
  delay_line: DelayLine,
  ramp: Ramp,
  previous_time: f32,
  next_time: f32,
}

impl VariableDelayLine {
  pub fn new(length: usize, sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new(length, sample_rate),
      ramp: Ramp::new(sample_rate),
      previous_time: 0.,
      next_time: 0.,
    }
  }

  pub fn read(&mut self, time: f32, interp: Interpolation) -> f32 {
    let time_has_changed = time != self.next_time;
    match (time_has_changed, self.ramp.is_finished()) {
      (false, true) => {
        let delay_out = self.delay_line.read(time, interp);
        self.next_time = time;
        delay_out
      }
      (true, true) => {
        self.previous_time = self.next_time;
        self.next_time = time;
        self.ramp.start(None);
        self.crossfade(interp)
      }
      _ => self.crossfade(interp),
    }
  }

  pub fn write(&mut self, value: f32) {
    self.delay_line.write(value);
  }

  fn crossfade(&mut self, interp: Interpolation) -> f32 {
    let ramp = self.ramp.process(5., 0., 1.);
    let window = (ramp * FRAC_PI_2).fast_cos();
    let window = window * window;
    self.delay_line.read(self.previous_time, interp) * window
      + self.delay_line.read(self.next_time, interp) * (1. - window)
  }
}
