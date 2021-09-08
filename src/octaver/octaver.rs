use super::clip::Clip;
use super::delta::Delta;
use super::lowpass::Lowpass;
use super::noise_gate::NoiseGate;

pub struct Octaver {
  lowpass: Lowpass,
  delta: Delta,
  noise_gate: NoiseGate,
  flip_flop: f32,
}

impl Octaver {
  pub fn new(sample_rate: f64) -> Self {
    Self {
      lowpass: Lowpass::new(sample_rate),
      delta: Delta::new(),
      noise_gate: NoiseGate::new(sample_rate),
      flip_flop: 1.,
    }
  }
  pub fn run(&mut self, input: f32, threshold: f32, gain: f32) -> f32 {
    let gated = self.noise_gate.run(input, threshold, 1.5, 30.);
    let lowpassed = self.lowpass.run(gated, 2.);
    let clipped = Clip::run(lowpassed * 10000., -1., 1.);
    let is_below_zero = if clipped < 0. { 1. } else { 0. };
    let trigger = self.delta.run(is_below_zero) > 0.;
    if trigger {
      if self.flip_flop == 1. {
        self.flip_flop = -1.
      } else {
        self.flip_flop = 1.
      }
    };
    clipped * self.flip_flop * gain
  }
}
