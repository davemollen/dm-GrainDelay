#[macro_use]
extern crate vst;
mod grain_delay_parameters;
use grain_delay::GrainDelay;
use grain_delay_parameters::GrainDelayParameters;
use std::sync::Arc;
use vst::{
  buffer::AudioBuffer,
  plugin::{Category, Info, Plugin, PluginParameters},
  prelude::HostCallback,
};

struct DmGrainDelay {
  params: Arc<GrainDelayParameters>,
  grain_delay: GrainDelay,
  is_active: bool,
}

impl Plugin for DmGrainDelay {
  fn new(_: HostCallback) -> Self {
    Self {
      params: Arc::new(GrainDelayParameters::default()),
      grain_delay: GrainDelay::new(44100.),
      is_active: false,
    }
  }

  fn set_sample_rate(&mut self, sample_rate: f32) {
    self.grain_delay = GrainDelay::new(sample_rate);
  }

  fn get_info(&self) -> Info {
    Info {
      name: "dm-GrainDelay".to_string(),
      vendor: "DM".to_string(),
      version: 1,
      inputs: 1,
      outputs: 2,
      parameters: 10,
      unique_id: 1358,
      f64_precision: true,
      category: Category::Effect,
      ..Default::default()
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    let spray = self.params.spray.get();
    let freq = self.params.frequency.get();
    let speed = 2_f32.powf(self.params.pitch.get() / 12.);
    let drift = self.params.drift.get();
    let drift = drift * drift;
    let reverse = self.params.reverse.get();
    let time = self.params.time.get();
    let feedback = self.params.feedback.get();
    let filter = self.params.filter.get();
    let spread = self.params.spread.get();
    let mix = self.params.mix.get();

    if !self.is_active {
      self
        .grain_delay
        .initialize_params(speed, filter, feedback, mix);
      self.is_active = true;
    }

    let (input_channels, mut output_channels) = buffer.split();
    let input = input_channels.get(0);
    let zipped_output_channels = output_channels
      .get_mut(0)
      .iter_mut()
      .zip(output_channels.get_mut(1).iter_mut());
    for (input, (output_left, output_right)) in input.iter().zip(zipped_output_channels) {
      let (grain_delay_left, grain_delay_right) = self.grain_delay.process(
        *input, spray, freq, speed, drift, reverse, time, feedback, filter, spread, mix,
      );
      *output_left = grain_delay_left;
      *output_right = grain_delay_right;
    }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }
}

plugin_main!(DmGrainDelay);
