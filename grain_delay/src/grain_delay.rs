use crate::delay_line::DelayLine;
use crate::delta::Delta;
use crate::grain::Grain;
use crate::lowpass::Lowpass;
use crate::mix::Mix;
use crate::phasor::Phasor;
use crate::MAX_GRAIN_DELAY_TIME;

const VOICES: usize = 4;

pub struct GrainDelay {
  grain_delay_line: DelayLine,
  delay_line: DelayLine,
  smooth_time: Lowpass,
  lowpass: Lowpass,
  phasor: Phasor,
  delta: Delta,
  grains: Vec<Grain>,
  index: usize,
}

impl GrainDelay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      grain_delay_line: DelayLine::new((sample_rate * MAX_GRAIN_DELAY_TIME) as usize, sample_rate),
      delay_line: DelayLine::new(sample_rate as usize * 5, sample_rate),
      smooth_time: Lowpass::new(sample_rate),
      lowpass: Lowpass::new(sample_rate),
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
      grains: vec![Grain::new(sample_rate); VOICES * 2],
      index: 0,
    }
  }

  fn grain_delay(&mut self, spray: f32, freq: f32, pitch: f32, drift: f32, reverse: f32) -> f32 {
    let Self {
      grains,
      grain_delay_line,
      ..
    } = self;

    let phasor_output = self.phasor.run(freq * VOICES as f32);
    let trigger = self.delta.run(phasor_output) < 0.;

    if trigger {
      let window_size = 1000. / freq;

      let (start, end) = grains.split_at(self.index);
      let index = start.iter().chain(end).position(|grain| grain.is_free());
      match index {
        Some(i) => {
          grains[i].set_parameters(freq, window_size, spray, drift, reverse);
          self.index = i;
        }
        None => {}
      }
    }

    grains
      .iter_mut()
      .filter(|grain| !grain.is_free())
      .map(|grain| grain.run(grain_delay_line, pitch))
      .sum()
  }

  pub fn run(
    &mut self,
    input: f32,
    spray: f32,
    freq: f32,
    pitch: f32,
    drift: f32,
    reverse: f32,
    delay_time: f32,
    feedback: f32,
    low_pass: f32,
    mix: f32,
  ) -> f32 {
    let time = self.smooth_time.run(delay_time, 3.);

    let delay_out = self.delay_line.read(time, "linear");
    let grain_delay_out = self.grain_delay(spray, freq, pitch, drift, reverse);
    self
      .delay_line
      .write(input + self.lowpass.run(grain_delay_out * 0.5 * feedback, low_pass));
    self.grain_delay_line.write(delay_out);
    Mix::run(input, grain_delay_out, mix)
  }
}
