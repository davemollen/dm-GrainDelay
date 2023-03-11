extern crate grain_delay;
extern crate lv2;
use grain_delay::GrainDelay;
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
  low_pass: InputPort<Control>,
  mix: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-GrainDelay")]
struct DmGrainDelay {
  grain_delay: GrainDelay,
}

impl Plugin for DmGrainDelay {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      grain_delay: GrainDelay::new(_plugin_info.sample_rate() as f32),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut ()) {
    let spray = *ports.spray;
    let freq = *ports.frequency;
    let pitch = *ports.pitch;
    let drift = *ports.drift;
    let reverse = *ports.reverse;
    let time = *ports.time;
    let feedback = *ports.feedback * 0.01;
    let low_pass = *ports.low_pass;
    let mix = *ports.mix * 0.01;

    for (in_frame, out_frame) in ports.input.iter().zip(ports.output.iter_mut()) {
      *out_frame = self.grain_delay.run(
        *in_frame, spray, freq, pitch, drift, reverse, time, feedback, low_pass, mix,
      );
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmGrainDelay);
