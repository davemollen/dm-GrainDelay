use crate::dc_block::DcBlock;
use crate::delay_line::{DelayLine, Interpolation};
use crate::delta::Delta;
use crate::grain::Grain;
use crate::mix::Mix;
use crate::one_pole_filter::{Mode, OnePoleFilter};
use crate::one_pole_filter_stereo::{Mode as StereoMode, OnePoleFilterStereo};
use crate::phasor::Phasor;
use crate::variable_delay::VariableDelay;
use crate::MAX_GRAIN_DELAY_TIME;
use std::f32::consts::FRAC_1_SQRT_2;

const VOICES: usize = 4;

pub struct GrainDelay {
  delay_line: DelayLine,
  variable_delay: VariableDelay,
  grain_delay_line: DelayLine,
  one_pole_filter: OnePoleFilterStereo,
  phasor: Phasor,
  delta: Delta,
  grains: Vec<Grain>,
  index: usize,
  dc_block: DcBlock,
  smooth_filter: OnePoleFilter,
  smooth_feedback: OnePoleFilter,
  smooth_mix: OnePoleFilter,
}

impl GrainDelay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new((sample_rate * 5.) as usize, sample_rate),
      variable_delay: VariableDelay::new(sample_rate),
      grain_delay_line: DelayLine::new(
        (sample_rate * MAX_GRAIN_DELAY_TIME).ceil() as usize,
        sample_rate,
      ),
      one_pole_filter: OnePoleFilterStereo::new(sample_rate),
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
      grains: vec![Grain::new(sample_rate); VOICES * 2],
      index: 0,
      dc_block: DcBlock::new(sample_rate),
      smooth_filter: OnePoleFilter::new(sample_rate),
      smooth_feedback: OnePoleFilter::new(sample_rate),
      smooth_mix: OnePoleFilter::new(sample_rate),
    }
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

    let (start, end) = self.grains.split_at(self.index);
    let index = start.iter().chain(end).position(|grain| grain.is_free());
    match index {
      Some(i) => {
        self.grains[i].set_parameters(freq, window_size, spray, pitch, drift, reverse, spread);
        self.index = i;
      }
      None => {}
    }
  }

  fn grain_delay(
    &mut self,
    spray: f32,
    freq: f32,
    pitch: f32,
    drift: f32,
    reverse: f32,
    spread: f32,
  ) -> (f32, f32) {
    let phasor_output = self.phasor.run(freq * VOICES as f32);
    let trigger = self.delta.run(phasor_output) < 0.;
    if trigger {
      self.set_grain_parameters(freq, spray, pitch, drift, reverse, spread);
    }

    let grain_delay_line = &mut self.grain_delay_line;
    self
      .grains
      .iter_mut()
      .filter(|grain| !grain.is_free())
      .map(|grain| grain.run(grain_delay_line, pitch))
      .fold((0., 0.), |sum, grain_out| {
        (sum.0 + grain_out.0, sum.1 + grain_out.1)
      })
  }

  fn apply_filter(&mut self, input: (f32, f32), pitch: f32, filter: f32) -> (f32, f32) {
    if filter > 0. {
      let is_low_pass_filter = pitch > 0.;
      if is_low_pass_filter {
        self
          .one_pole_filter
          .run(input, filter.powf(0.33333), StereoMode::Linear)
      } else {
        let filter_out = self
          .one_pole_filter
          .run(input, 1. - filter.powf(3.), StereoMode::Linear);
        (filter_out.0 - input.0, filter_out.1 - input.1)
      }
    } else {
      input
    }
  }

  fn apply_feedback(&mut self, input: (f32, f32), feedback: f32) -> f32 {
    let mono_input = (input.0 + input.1) * FRAC_1_SQRT_2;
    let feedback_out = mono_input * 0.5 * feedback;
    self.dc_block.run(feedback_out.clamp(-1., 1.))
  }

  pub fn run(
    &mut self,
    input: f32,
    spray: f32,
    freq: f32,
    pitch: f32,
    drift: f32,
    reverse: f32,
    time: f32,
    feedback: f32,
    filter: f32,
    spread: f32,
    mix: f32,
  ) -> (f32, f32) {
    let filter = self.smooth_filter.run(filter, 12., Mode::Hertz);
    let feedback = self.smooth_feedback.run(feedback, 12., Mode::Hertz);
    let mix = self.smooth_mix.run(mix, 12., Mode::Hertz);

    let delay_out = self
      .variable_delay
      .read(&mut self.delay_line, time, Interpolation::Linear);
    let grain_delay_out = self.grain_delay(spray, freq, pitch, drift, reverse, spread);
    let filter_out = self.apply_filter(grain_delay_out, pitch, filter);
    let feedback_out = self.apply_feedback(filter_out, feedback);

    self.delay_line.write(input + feedback_out);
    self.grain_delay_line.write(delay_out);
    Mix::run(input, filter_out, mix)
  }
}
