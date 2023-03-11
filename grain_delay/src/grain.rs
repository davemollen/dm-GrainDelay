use crate::{delay_line::DelayLine, ramp::Ramp};
use rand::random;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Grain {
  freq: f32,
  drift: f32,
  start_position: f32,
  window_size: f32,
  time_ramp: Ramp,
  window_ramp: Ramp,
  is_reversed: bool,
}

impl Grain {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      freq: 7.,
      drift: 0.,
      start_position: 0.,
      window_size: 1000.,
      time_ramp: Ramp::new(sample_rate),
      window_ramp: Ramp::new(sample_rate),
      is_reversed: false,
    }
  }

  pub fn set_parameters(
    &mut self,
    freq: f32,
    window_size: f32,
    spray: f32,
    drift: f32,
    reverse: f32,
    time: f32,
  ) {
    self.freq = freq;
    self.window_size = window_size;
    self.is_reversed = random::<f32>() <= reverse;
    self.start_position = random::<f32>() * spray + time;
    self.drift = random::<f32>() * drift * 2. - drift;
    self.time_ramp.start(None);
    self.window_ramp.start(None);
  }

  pub fn is_free(&self) -> bool {
    self.window_ramp.is_finished()
  }

  pub fn run(&mut self, grain_delay_line: &mut DelayLine, pitch: f32) -> f32 {
    let speed = 2_f32.powf((pitch + self.drift) / 12.);
    let ramp_freq = if self.is_reversed {
      (1. + speed) * self.freq
    } else {
      (1. - speed) * self.freq
    };
    let time = self.time_ramp.run(ramp_freq, false) * self.window_size;
    let grain_delay_line_out = grain_delay_line.read(time + self.start_position, "linear");
    let windowing = ((self.window_ramp.run(self.freq, true) - 0.5) * PI).cos();
    grain_delay_line_out * windowing * windowing
  }
}
