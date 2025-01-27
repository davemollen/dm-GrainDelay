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
  drift: f32,
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
      drift: 1.,
      is_reversed: false,
    }
  }

  pub fn process(
    &mut self,
    grain_delay_line: &DelayLine,
    phasor: f32,
    freq: f32,
    speed: f32,
    spray: f32,
    drift: f32,
    reverse: f32,
    pan: f32,
  ) -> (f32, f32) {
    let phase = Self::wrap(phasor + self.phase_offset);
    let trigger = self.delta.process(phase) < 0.;
    if trigger {
      self.set_grain_params(freq, spray, drift, reverse, pan);
    }

    let (ramp, time) = self.get_ramp_and_time(speed);
    let window = (ramp * PI).fast_sin() * (phase * PI).fast_sin();
    let grains_out =
      grain_delay_line.read(time + self.start_position, Interpolation::Linear) * window;
    grains_out.pan(self.pan)
  }

  fn set_grain_params(&mut self, freq: f32, spray: f32, drift: f32, reverse: f32, pan: f32) {
    self.freq = freq;
    self.time_ramp.start();
    self.start_position = fastrand::f32() * spray;
    self.pan = (fastrand::f32() * pan * 2. - pan) * 50.;
    self.is_reversed = fastrand::f32() <= reverse;
    self.window_size = freq.recip() * 1000.;
    self.set_drift(drift);
  }

  fn get_speed_for_delay_line(&self, speed: f32) -> f32 {
    if self.is_reversed {
      1. + speed
    } else {
      1. - speed
    }
  }

  fn set_drift(&mut self, drift: f32) {
    let random_pitch = fastrand::f32() * drift * 2. - drift;
    self.drift = 2_f32.powf(random_pitch / 12.);
  }

  fn get_ramp_and_time(&mut self, speed: f32) -> (f32, f32) {
    let speed = self.get_speed_for_delay_line(speed * self.drift);
    let ramp = self
      .time_ramp
      .process(if speed >= 0. { self.freq } else { -self.freq });
    (ramp, ramp * speed.abs() * self.window_size)
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
