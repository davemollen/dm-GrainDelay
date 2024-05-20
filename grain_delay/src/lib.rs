include!(concat!(env!("OUT_DIR"), "/constants.rs"));
mod dc_block;
mod shared {
  pub mod delay_line;
  pub mod float_ext;
}
mod grains;
mod mix;
mod one_pole_filter_stereo;
mod param_filter;
mod variable_delay_line;
use {
  dc_block::DcBlock, grains::Grains, mix::Mix, one_pole_filter_stereo::OnePoleFilterStereo,
  param_filter::ParamFilter, shared::delay_line::Interpolation,
  variable_delay_line::VariableDelayLine,
};

pub struct GrainDelay {
  variable_delay_line: VariableDelayLine,
  low_pass_filter: OnePoleFilterStereo,
  grains: Grains,
  dc_block: DcBlock,
  smooth_speed: ParamFilter,
  smooth_filter: ParamFilter,
  smooth_feedback: ParamFilter,
  smooth_mix: ParamFilter,
}

impl GrainDelay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      variable_delay_line: VariableDelayLine::new((sample_rate * 5.) as usize, sample_rate),
      low_pass_filter: OnePoleFilterStereo::new(sample_rate),
      grains: Grains::new(sample_rate),
      dc_block: DcBlock::new(sample_rate),
      smooth_speed: ParamFilter::new(sample_rate, 12.),
      smooth_filter: ParamFilter::new(sample_rate, 12.),
      smooth_feedback: ParamFilter::new(sample_rate, 12.),
      smooth_mix: ParamFilter::new(sample_rate, 12.),
    }
  }

  pub fn initialize_params(&mut self, pitch: f32, filter: f32, feedback: f32, mix: f32) {
    self.smooth_speed.initialize(pitch);
    self.smooth_filter.initialize(filter);
    self.smooth_feedback.initialize(feedback);
    self.smooth_mix.initialize(mix);
  }

  pub fn process(
    &mut self,
    input: f32,
    spray: f32,
    freq: f32,
    speed: f32,
    drift: f32,
    reverse: f32,
    time: f32,
    feedback: f32,
    filter: f32,
    spread: f32,
    mix: f32,
  ) -> (f32, f32) {
    let speed = self.smooth_speed.process(speed);
    let filter = self.smooth_filter.process(filter);
    let feedback = self.smooth_feedback.process(feedback);
    let mix = self.smooth_mix.process(mix);

    let delay_out = self.variable_delay_line.read(time, Interpolation::Step);
    let grain_delay_out = self
      .grains
      .process(delay_out, spray, freq, speed, drift, reverse, spread);
    let filter_out = self.low_pass_filter.process(grain_delay_out, filter);
    let feedback_out = self.apply_feedback(filter_out, feedback);
    self.variable_delay_line.write(input + feedback_out);

    Mix::process(input, filter_out, mix)
  }

  fn apply_feedback(&mut self, input: (f32, f32), feedback: f32) -> f32 {
    let mono_input = (input.0 + input.1) * 0.5;
    let feedback_out = mono_input * feedback;
    self.dc_block.process(feedback_out.clamp(-1., 1.))
  }
}
