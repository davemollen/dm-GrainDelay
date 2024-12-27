include!(concat!(env!("OUT_DIR"), "/constants.rs"));
mod dc_block;
mod shared {
  pub mod delay_line;
  pub mod float_ext;
}
mod grains;
mod mix;
mod one_pole_filter_stereo;
mod params;
mod variable_delay_line;
pub use params::Params;
use {
  dc_block::DcBlock, grains::Grains, mix::Mix, one_pole_filter_stereo::OnePoleFilterStereo,
  params::Smoother, shared::delay_line::Interpolation, variable_delay_line::VariableDelayLine,
};

pub struct GrainDelay {
  variable_delay_line: VariableDelayLine,
  low_pass_filter: OnePoleFilterStereo,
  grains: Grains,
  dc_block: DcBlock,
}

impl GrainDelay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      variable_delay_line: VariableDelayLine::new((sample_rate * 5.) as usize, sample_rate),
      low_pass_filter: OnePoleFilterStereo::new(sample_rate),
      grains: Grains::new(sample_rate),
      dc_block: DcBlock::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> (f32, f32) {
    let Params {
      spray,
      freq,
      drift,
      reverse,
      time,
      spread,
      ..
    } = *params;
    let speed = params.speed.next();
    let feedback = params.feedback.next();
    let filter = params.filter.next();
    let mix = params.mix.next();

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
