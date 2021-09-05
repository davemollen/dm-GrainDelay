mod delta;
use self::delta::Delta;
mod lowpass;
use self::lowpass::Lowpass;
mod noise_gate;
use self::noise_gate::NoiseGate;

pub struct Octaver {
  lowpass: Lowpass,
  delta: Delta,
  noise_gate: NoiseGate,
  flip_flop: f32,
}

impl Octaver {
  pub fn new(sample_rate: f64) -> Self {
    Self {
      lowpass: Lowpass::new(),
      delta: Delta::new(),
      noise_gate: NoiseGate::new(sample_rate),
      flip_flop: 1.,
    }
  }
  pub fn run(&mut self, input: f32, threshold: f32, gain: f32) -> f32 {
    let gate = self.noise_gate.run(input, threshold, 2., 120.);
    let amplify = self.lowpass.run(gate, 0.9997) * 10000.;
    let clip = if amplify > 1. {
      1.
    } else if amplify < -1. {
      -1.
    } else {
      amplify
    };

    let is_below_zero = if clip < 0. { 1. } else { 0. };
    let trigger = self.delta.run(is_below_zero) > 0.;
    if trigger {
      if self.flip_flop == 1. {
        self.flip_flop = -1.
      } else {
        self.flip_flop = 1.
      }
    };
    clip * self.flip_flop * gain
  }
}
