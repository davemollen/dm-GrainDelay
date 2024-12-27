extern crate grain_delay;
extern crate lv2;
use grain_delay::{GrainDelay, Params};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  spray: InputPort<Control>,
  frequency: InputPort<Control>,
  pitch: InputPort<Control>,
  drift: InputPort<Control>,
  reverse: InputPort<Control>,
  time: InputPort<Control>,
  feedback: InputPort<Control>,
  filter: InputPort<Control>,
  spread: InputPort<Control>,
  mix: InputPort<Control>,
  input: InputPort<Audio>,
  output_left: OutputPort<Audio>,
  output_right: OutputPort<Audio>,
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
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = _plugin_info.sample_rate() as f32;

    Some(Self {
      grain_delay: GrainDelay::new(sample_rate),
      params: Params::new(sample_rate),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.params.set(
      *ports.spray,
      *ports.frequency,
      *ports.pitch,
      *ports.drift * 0.01,
      *ports.reverse * 0.01,
      *ports.time,
      *ports.feedback * 0.01,
      *ports.filter,
      *ports.spread * 0.01,
      *ports.mix * 0.01,
    );

    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());
    for (input, (out_left, out_right)) in ports.input.iter().zip(output_channels) {
      (*out_left, *out_right) = self.grain_delay.process(*input, &mut self.params);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmGrainDelay);
