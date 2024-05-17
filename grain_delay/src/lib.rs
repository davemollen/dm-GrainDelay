include!(concat!(env!("OUT_DIR"), "/constants.rs"));
mod dc_block;
mod delay_line;
mod delta;
mod float_ext;
mod grain;
mod mix;
mod one_pole_filter_stereo;
mod param_filter;
mod phasor;
mod variable_delay_line;
use {
  dc_block::DcBlock,
  delay_line::{DelayLine, Interpolation},
  delta::Delta,
  grain::Grain,
  mix::Mix,
  one_pole_filter_stereo::OnePoleFilterStereo,
  param_filter::ParamFilter,
  phasor::Phasor,
  variable_delay_line::VariableDelayLine,
};

const VOICES: usize = 4;

pub struct GrainDelay {
  variable_delay_line: VariableDelayLine,
  grain_delay_line: DelayLine,
  low_pass_filter: OnePoleFilterStereo,
  phasor: Phasor,
  delta: Delta,
  grains: Vec<Grain>,
  dc_block: DcBlock,
  smooth_pitch: ParamFilter,
  smooth_filter: ParamFilter,
  smooth_feedback: ParamFilter,
  smooth_mix: ParamFilter,
}

impl GrainDelay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      variable_delay_line: VariableDelayLine::new((sample_rate * 5.) as usize, sample_rate),
      grain_delay_line: DelayLine::new((sample_rate * MAX_GRAIN_DELAY_TIME) as usize, sample_rate),
      low_pass_filter: OnePoleFilterStereo::new(sample_rate),
      phasor: Phasor::new(sample_rate),
      grains: vec![Grain::new(sample_rate); VOICES * 2],
      delta: Delta::new(),
      dc_block: DcBlock::new(sample_rate),
      smooth_pitch: ParamFilter::new(sample_rate, 12.),
      smooth_filter: ParamFilter::new(sample_rate, 12.),
      smooth_feedback: ParamFilter::new(sample_rate, 12.),
      smooth_mix: ParamFilter::new(sample_rate, 12.),
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
    time: f32,
    feedback: f32,
    filter: f32,
    spread: f32,
    mix: f32,
  ) -> (f32, f32) {
    let pitch = self.smooth_pitch.process(pitch);
    let filter = self.smooth_filter.process(filter);
    let feedback = self.smooth_feedback.process(feedback);
    let mix = self.smooth_mix.process(mix);

    let delay_out = self.variable_delay_line.read(time, Interpolation::Step);
    let grain_delay_out = self.grain_delay(spray, freq, pitch, drift, reverse, spread);
    let filter_out = self.apply_filter(grain_delay_out, filter);
    let feedback_out = self.apply_feedback(filter_out, feedback);

    self.variable_delay_line.write(input + feedback_out);
    self.grain_delay_line.write(delay_out);
    Mix::process(input, filter_out, mix)
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
    let phasor_output = self.phasor.process(freq * VOICES as f32);
    let trigger = self.delta.process(phasor_output) < 0.;
    if trigger {
      self.set_grain_parameters(freq, spray, pitch, drift, reverse, spread);
    }

    let grain_delay_line = &mut self.grain_delay_line;
    self
      .grains
      .iter_mut()
      .filter(|grain| !grain.is_free())
      .map(|grain| grain.process(grain_delay_line, pitch))
      .fold((0., 0.), |sum, grain_out| {
        (sum.0 + grain_out.0, sum.1 + grain_out.1)
      })
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

  fn apply_filter(&mut self, input: (f32, f32), filter: f32) -> (f32, f32) {
    self.low_pass_filter.process(input, filter)
  }

  fn apply_feedback(&mut self, input: (f32, f32), feedback: f32) -> f32 {
    let mono_input = (input.0 + input.1) * 0.5;
    let feedback_out = mono_input * feedback;
    self.dc_block.process(feedback_out.clamp(-1., 1.))
  }
}
