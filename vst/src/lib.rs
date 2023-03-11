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
}

impl Plugin for DmGrainDelay {
  fn new(_: HostCallback) -> Self {
    Self {
      params: Arc::new(GrainDelayParameters::default()),
      grain_delay: GrainDelay::new(44100.),
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
      outputs: 1,
      parameters: 9,
      unique_id: 1358,
      f64_precision: true,
      category: Category::Effect,
      ..Default::default()
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    let spray = self.params.spray.get();
    let freq = self.params.frequency.get();
    let pitch = self.params.pitch.get();
    let drift = self.params.drift.get();
    let reverse = self.params.reverse.get();
    let time = self.params.time.get();
    let feedback = self.params.feedback.get();
    let low_pass = self.params.low_pass.get();
    let mix = self.params.mix.get();

    for (input_buffer, output_buffer) in buffer.zip() {
      for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
        *output_sample = self.grain_delay.run(
          *input_sample,
          spray,
          freq,
          pitch,
          drift,
          reverse,
          time,
          feedback,
          low_pass,
          mix,
        );
      }
    }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }
}

plugin_main!(DmGrainDelay);
