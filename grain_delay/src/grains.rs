mod grain;
mod phasor;
use crate::{shared::delay_line::DelayLine, MAX_GRAIN_DELAY_TIME};
use {grain::Grain, phasor::Phasor};

const VOICES: usize = 4;

pub struct Grains {
  grain_delay_line: DelayLine,
  phasor: Phasor,
  grains: Vec<Grain>,
  gain_correction: f32,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    let grains = (0..VOICES).map(|i| Grain::new(sample_rate, i)).collect();

    Self {
      grain_delay_line: DelayLine::new((sample_rate * MAX_GRAIN_DELAY_TIME) as usize, sample_rate),
      phasor: Phasor::new(sample_rate),
      grains,
      gain_correction: (VOICES as f32 / 2.).recip(),
    }
  }

  pub fn process(
    &mut self,
    input: f32,
    spray: f32,
    freq: f32,
    speed: f32,
    drift: f32,
    reverse: f32,
    spread: f32,
  ) -> (f32, f32) {
    let phasor = self.phasor.process(freq);

    let grains_out = self
      .grains
      .iter_mut()
      .map(|grain| {
        grain.process(
          &self.grain_delay_line,
          phasor,
          freq,
          speed,
          spray,
          drift,
          reverse,
          spread,
        )
      })
      .fold((0., 0.), |sum, grain_out| {
        (sum.0 + grain_out.0, sum.1 + grain_out.1)
      });

    self.grain_delay_line.write(input);

    (
      grains_out.0 * self.gain_correction,
      grains_out.1 * self.gain_correction,
    )
  }
}
