mod delta;
mod pan;
mod ramp;
use super::VOICES;
use crate::shared::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
};
use std::f32::consts::PI;
use {delta::Delta, pan::Pan, ramp::Ramp};

#[derive(Clone)]
pub struct Grain {
  freq: f32,
  start_position: f32,
  pan: f32,
  window_size: f32,
  time_ramp: Ramp,
  phase_offset: f32,
  delta: Delta,
  time_multiplier: f32,
  is_reversed: bool,
}

impl Grain {
  pub fn new(sample_rate: f32, index: usize) -> Self {
    Self {
      freq: 0.,
      start_position: 0.,
      pan: 0.,
      window_size: 0.,
      time_ramp: Ramp::new(sample_rate),
      phase_offset: (VOICES as f32).recip() * index as f32,
      delta: Delta::new(),
      time_multiplier: 1.,
      is_reversed: false,
    }
  }

  pub fn process(
    &mut self,
    grain_delay_line: &mut DelayLine,
    phasor: f32,
    freq: f32,
    speed: f32,
    spray: f32,
    drift: f32,
    reverse: f32,
    pan: f32,
  ) -> (f32, f32) {
    let phase = Self::wrap(phasor + self.phase_offset);
    let trigger = self.delta.process(phase).abs() > 0.5;
    if trigger {
      self.set_grain_params(freq, speed, spray, drift, reverse, pan);
    }

    let (ramp, time) = self.get_ramp_and_time(speed);
    let window = (ramp * PI).fast_sin() * (phase * PI).fast_sin();
    let grains_out =
      grain_delay_line.read(time + self.start_position, Interpolation::Linear) * window;
    grains_out.pan(self.pan)
  }

  fn set_grain_params(
    &mut self,
    freq: f32,
    speed: f32,
    spray: f32,
    drift: f32,
    reverse: f32,
    pan: f32,
  ) {
    self.freq = freq;
    self.time_ramp.start();
    self.start_position = fastrand::f32() * spray;
    self.pan = (fastrand::f32() * pan * 2. - pan) * 50.;
    self.is_reversed = fastrand::f32() <= reverse;
    self.window_size = freq.recip() * 1000.;
    self.set_time_multiplier(speed, drift);
  }

  fn get_speed_for_delay_line(&self, speed: f32) -> f32 {
    if self.is_reversed {
      1. + speed
    } else {
      1. - speed
    }
  }

  fn get_drift(&mut self, drift: f32) -> f32 {
    let random_pitch = fastrand::f32() * drift * 2. - drift;
    2_f32.powf(random_pitch / 12.)
  }

  fn set_time_multiplier(&mut self, speed: f32, drift: f32) {
    let drift = self.get_drift(drift);
    self.time_multiplier = self.get_speed_for_delay_line(speed * drift).abs();
  }

  fn get_ramp_and_time(&mut self, speed: f32) -> (f32, f32) {
    if self.time_multiplier == 0. {
      let ramp = self.time_ramp.process(self.freq);
      (
        ramp,
        Self::wrap(ramp * self.get_speed_for_delay_line(speed)) * self.window_size,
      )
    } else {
      let ramp_freq = self.get_speed_for_delay_line(speed) * self.freq;
      let ramp = self
        .time_ramp
        .process(ramp_freq * self.time_multiplier.recip());
      (ramp, ramp * self.time_multiplier * self.window_size)
    }
  }

  fn wrap(input: f32) -> f32 {
    if input >= 1. {
      input - 1.
    } else if input < 0. {
      input + 1.
    } else {
      input
    }
  }
}
