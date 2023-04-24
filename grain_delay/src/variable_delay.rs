use crate::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
  ramp::Ramp,
};
use std::f32::consts::FRAC_PI_2;

pub struct VariableDelay {
  ramp: Ramp,
  previous_time: f32,
  next_time: f32,
}

impl VariableDelay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      ramp: Ramp::new(sample_rate),
      previous_time: 0.,
      next_time: 0.,
    }
  }

  fn crossfade(&mut self, delay_line: &mut DelayLine, interp: Interpolation) -> f32 {
    let ramp = self.ramp.run(5., 0., 1.);
    let window = (ramp * FRAC_PI_2).fast_cos();
    let window = window * window;
    delay_line.read(self.previous_time, interp) * window
      + delay_line.read(self.next_time, interp) * (1. - window)
  }

  pub fn read(&mut self, delay_line: &mut DelayLine, time: f32, interp: Interpolation) -> f32 {
    let time_has_changed = time != self.next_time;
    match (time_has_changed, self.ramp.is_finished()) {
      (false, true) => {
        let delay_out = delay_line.read(time, interp);
        self.next_time = time;
        delay_out
      }
      (true, true) => {
        self.previous_time = self.next_time;
        self.next_time = time;
        self.ramp.start(None);
        self.crossfade(delay_line, interp)
      }
      _ => self.crossfade(delay_line, interp),
    }
  }
}
