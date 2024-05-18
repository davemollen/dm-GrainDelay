mod grain;
mod phasor;
use crate::{
  shared::{delay_line::DelayLine, delta::Delta},
  MAX_GRAIN_DELAY_TIME, VOICES,
};
use {grain::Grain, phasor::Phasor};

pub struct Grains {
  grain_delay_line: DelayLine,
  phasor: Phasor,
  delta: Delta,
  grains: Vec<Grain>,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      grain_delay_line: DelayLine::new((sample_rate * MAX_GRAIN_DELAY_TIME) as usize, sample_rate),
      phasor: Phasor::new(sample_rate),
      grains: vec![Grain::new(sample_rate); VOICES * 2],
      delta: Delta::new(),
    }
  }

  pub fn process(
    &mut self,
    input: f32,
    spray: f32,
    freq: f32,
    pitch: f32,
    drift: f32,
    reverse: f32,
    spread: f32,
  ) -> (f32, f32) {
    let phasor = self.phasor.process(freq * VOICES as f32);
    let trigger = self.delta.process(phasor) < 0.;
    if trigger {
      self.set_grain_parameters(freq, spray, pitch, drift, reverse, spread);
    }

    let grain_delay_line = &mut self.grain_delay_line;
    let grains_out = self
      .grains
      .iter_mut()
      .filter(|grain| !grain.is_free())
      .map(|grain| grain.process(grain_delay_line, pitch))
      .fold((0., 0.), |sum, grain_out| {
        (sum.0 + grain_out.0, sum.1 + grain_out.1)
      });

    self.grain_delay_line.write(input);

    grains_out
  }

  fn set_grain_parameters(
    &mut self,
    freq: f32,
    spray: f32,
    pitch: f32,
    drift: f32,
    reverse: f32,
    spread: f32,
  ) {
    let window_size = 1000. / freq;

    let grain = self.grains.iter_mut().find(|grain| grain.is_free());
    match grain {
      Some(g) => {
        g.set_parameters(freq, window_size, spray, pitch, drift, reverse, spread);
      }
      None => {}
    }
  }
}
