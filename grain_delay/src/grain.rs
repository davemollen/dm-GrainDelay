mod pan;
mod ramp;
use crate::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
};
use std::f32::consts::PI;
use {pan::Pan, ramp::Ramp};

#[derive(Clone)]
pub struct Grain {
  freq: f32,
  drift: f32,
  start_position: f32,
  pan: f32,
  window_size: f32,
  time_ramp: Ramp,
  is_reversed: bool,
  time_ramp_max: f32,
}

impl Grain {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      freq: 0.,
      drift: 0.,
      start_position: 0.,
      pan: 0.,
      window_size: 0.,
      time_ramp: Ramp::new(sample_rate),
      is_reversed: false,
      time_ramp_max: 1.,
    }
  }

  pub fn is_free(&self) -> bool {
    self.time_ramp.is_finished()
  }

  pub fn set_parameters(
    &mut self,
    freq: f32,
    window_size: f32,
    spray: f32,
    pitch: f32,
    drift: f32,
    reverse: f32,
    pan: f32,
  ) {
    self.freq = freq;
    self.window_size = window_size;
    self.is_reversed = fastrand::f32() <= reverse;
    self.start_position = fastrand::f32() * spray;
    self.pan = (fastrand::f32() * pan * 2. - pan) * 50.;
    let exponential_drift = drift.fast_pow(2.);
    self.drift = fastrand::f32() * exponential_drift * 2. - exponential_drift;
    self.time_ramp.start(None);

    let speed = 2_f32.powf((pitch + self.drift) / 12.);
    self.time_ramp_max = if self.is_reversed {
      (1. + speed) * freq
    } else {
      (1. - speed) * freq
    }
    .abs()
      / freq;
  }

  pub fn process(&mut self, grain_delay_line: &mut DelayLine, pitch: f32) -> (f32, f32) {
    let speed = 2_f32.powf((pitch + self.drift) / 12.);
    let time = self.get_time(speed);
    let window = self.get_window();

    let grains_out = grain_delay_line.read(time + self.start_position, Interpolation::Linear);
    let windowed_grains_out = grains_out * window * window;
    windowed_grains_out.pan(self.pan)
  }

  fn get_time(&mut self, speed: f32) -> f32 {
    if self.time_ramp_max == 0. {
      self.time_ramp.process(self.freq, 0., 1.);
      0.
    } else {
      let ramp_freq = if self.is_reversed {
        (1. + speed) * self.freq
      } else {
        (1. - speed) * self.freq
      };

      self.time_ramp.process(ramp_freq, 0., self.time_ramp_max) * self.window_size
    }
  }

  fn get_window(&mut self) -> f32 {
    (self.time_ramp.get_progress() * PI).fast_sin()
  }
}
