extern crate grain_delay;
extern crate lv2;
use grain_delay::{Bypass, GrainDelay, Params};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  spray: InputPort<InPlaceControl>,
  frequency: InputPort<InPlaceControl>,
  pitch: InputPort<InPlaceControl>,
  drift: InputPort<InPlaceControl>,
  reverse: InputPort<InPlaceControl>,
  time: InputPort<InPlaceControl>,
  feedback: InputPort<InPlaceControl>,
  filter: InputPort<InPlaceControl>,
  spread: InputPort<InPlaceControl>,
  mix: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output_left: OutputPort<InPlaceAudio>,
  output_right: OutputPort<InPlaceAudio>,
  enabled: InputPort<InPlaceControl>,
}

#[uri("https://github.com/davemollen/dm-GrainDelay")]
struct DmGrainDelay {
  grain_delay: GrainDelay,
  params: Params,
  bypass: Bypass,
}

impl Plugin for DmGrainDelay {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      grain_delay: GrainDelay::new(sample_rate),
      params: Params::new(sample_rate),
      bypass: Bypass::new(sample_rate),
    })
  }

  /// Reset the internal state of the plugin.
  fn activate(&mut self, _features: &mut Self::InitFeatures) {
    self.grain_delay.reset();
    self.params.reset();
    self.bypass.reset();
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), sample_count: u32) {
    self.bypass.set_enabled(ports.enabled.get() == 0., || {
      self.grain_delay.reset();
      self.params.reset();
    });

    if self.bypass.is_done_activating() {
      ports.output_left.copy_from(&ports.input, sample_count);
      ports.output_right.copy_from(&ports.input, sample_count);
      return;
    }

    self.params.set(
      ports.spray.get(),
      ports.frequency.get(),
      ports.pitch.get(),
      ports.drift.get() * 0.01,
      ports.reverse.get() * 0.01,
      ports.time.get(),
      ports.feedback.get() * 0.01,
      ports.filter.get(),
      ports.spread.get() * 0.01,
      ports.mix.get() * 0.01,
    );

    let output_channels = ports.output_left.iter().zip(ports.output_right.iter());
    for (input, (output_left, output_right)) in ports.input.iter().zip(output_channels) {
      let dry = input.get();
      let wet = self.grain_delay.process(dry, &mut self.params);
      let grain_delay_output = self.bypass.process(dry, wet);
      output_left.set(grain_delay_output.0);
      output_right.set(grain_delay_output.1);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmGrainDelay);
