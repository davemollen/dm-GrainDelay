use super::delay_line::DelayLine;
use super::delta::Delta;
use super::lowpass::Lowpass;
use super::mix::Mix;
use super::phasor::Phasor;
use rand::Rng;
use std::f32;

pub struct GrainDelay {
  pitchshift: DelayLine,
  delay: DelayLine,
  smooth_time: Lowpass,
  smooth_frequency: Lowpass,
  lowpass: Lowpass,
  phasor: Phasor,
  delta: Vec<Delta>,
  start_position: Vec<f32>,
  rand_pitch: Vec<f32>,
}

impl GrainDelay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      pitchshift: DelayLine::new((sample_rate * 1.5) as usize, sample_rate),
      delay: DelayLine::new(sample_rate as usize * 5, sample_rate),
      smooth_time: Lowpass::new(sample_rate),
      smooth_frequency: Lowpass::new(sample_rate),
      lowpass: Lowpass::new(sample_rate),
      phasor: Phasor::new(sample_rate),
      delta: vec![Delta::new(); 4],
      start_position: vec![0.0; 4],
      rand_pitch: vec![0.0; 4],
    }
  }

  fn grain_delay(&mut self, spray: f32, frequency: f32, pitch: f32, rand_pitch: f32) -> f32 {
    let speed = f32::powf(2., pitch / 12.);
    let window_size = 1000. / frequency;
    let phasor_freq = (1. - speed) * frequency;
    let main_phasor = self.phasor.run(phasor_freq);
    let mut out = 0f32;

    for i in 0..4 {
      let phasor = (main_phasor + (i as f32 / 4.)) % 1.;
      let delta = self.delta[i].run(phasor);
      let trigger = if phasor_freq > 0. {
        delta < 0.
      } else {
        delta > 0.
      };
      if trigger {
        let noise: f32 = rand::thread_rng().gen();
        self.start_position[i] = noise * spray;
        self.rand_pitch[i] = noise * rand_pitch * 0.2 + 1.;
      };
      let windowing = ((phasor - 0.5) * f32::consts::PI).cos();
      out += self.pitchshift.read(
        phasor * window_size * self.rand_pitch[i] + self.start_position[i],
        "linear",
      ) * windowing
        * windowing;
    }
    out
  }

  pub fn run(
    &mut self,
    input: f32,
    spray: f32,
    frequency: f32,
    pitch: f32,
    rand_pitch: f32,
    delay_time: f32,
    feedback: f32,
    low_pass: f32,
    mix: f32,
  ) -> f32 {
    let time = self.smooth_time.run(delay_time, 3.);
    let freq = self.smooth_frequency.run(frequency, 3.);

    let delay = self.delay.read(time, "linear");
    let pitchshift = self.grain_delay(spray, freq, pitch, rand_pitch);
    self
      .delay
      .write(input + self.lowpass.run(pitchshift * 0.5 * feedback, low_pass));
    self.pitchshift.write(delay);
    Mix::run(input, pitchshift, mix)
  }
}
