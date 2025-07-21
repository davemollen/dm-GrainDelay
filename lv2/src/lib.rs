extern crate grain_delay;
extern crate lv2;
use grain_delay::{GrainDelay, Params};
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
}

#[uri("https://github.com/davemollen/dm-GrainDelay")]
struct DmGrainDelay {
  grain_delay: GrainDelay,
  params: Params,
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
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
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
      let output = self.grain_delay.process(input.get(), &mut self.params);
      output_left.set(output.0);
      output_right.set(output.1);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmGrainDelay);
