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
  phasor: Phasor,
  lowpass: Lowpass,
  delta: Vec<Delta>,
  start_position: Vec<f32>,
  rand_pitch: Vec<f32>,
}

impl GrainDelay {
  pub fn new(sample_rate: f64) -> Self {
    Self {
      pitchshift: DelayLine::new(sample_rate as usize, sample_rate),
      delay: DelayLine::new(sample_rate as usize * 5, sample_rate),
      phasor: Phasor::new(sample_rate),
      lowpass: Lowpass::new(sample_rate),
      delta: {
        let mut vec = Vec::new();
        for i in 0..4 {
          vec[i] = Delta::new();
        }
        vec
      },
      start_position: vec![0.; 4],
      rand_pitch: vec![0.; 4],
    }
  }

  fn grain_delay(&mut self, pitch: f32, spray: f32, frequency: f32, rand_pitch: f32) -> f32 {
    let speed = f32::powf(2., pitch / 12.);
    let window_size = 1000. / frequency;
    let phasor_freq = (1. - speed) * frequency;
    let main_phasor = self.phasor.run(phasor_freq);
    let mut grain_delay_out = 0.;

    for index in 0..4 {
      let phasor = (main_phasor + 1. / index as f32) % 1.;
      let trigger = self.delta[index].run(phasor) > 0.;
      if trigger {
        let noise: f32 = rand::thread_rng().gen();
        self.start_position[index] = noise * spray;
        self.rand_pitch[index] = noise * rand_pitch * 0.1;
      };
      let windowing = ((phasor - 0.5) * f32::consts::PI).cos();
      grain_delay_out += self.pitchshift.read(
        phasor * window_size * rand_pitch + self.start_position[index],
        "linear",
      ) * (windowing * windowing);
    }
    grain_delay_out
  }
  pub fn run(
    &mut self,
    input: f32,
    pitch: f32,
    spray: f32,
    frequency: f32,
    rand_pitch: f32,
    delay_time: f32,
    feedback: f32,
    mix: f32,
  ) -> f32 {
    let time = self.lowpass.run(delay_time, 0.3);
    let delay = self.delay.read(time, "linear");
    let pitchshift = self.grain_delay(pitch, spray, frequency, rand_pitch);
    self.delay.write(input + pitchshift * feedback);
    self.pitchshift.write(delay);
    Mix::run(input, pitchshift, mix)
  }
}
