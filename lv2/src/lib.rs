extern crate grain_delay;
extern crate lv2;
use grain_delay::{FloatExt, GrainDelay};
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
  is_active: bool,
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
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let spray = *ports.spray;
    let freq = *ports.frequency;
    let pitch = *ports.pitch;
    let drift = (*ports.drift * 0.01).fast_pow(2.);
    let reverse = *ports.reverse * 0.01;
    let time = *ports.time;
    let feedback = *ports.feedback * 0.01;
    let filter = *ports.filter;
    let spread = *ports.spread * 0.01;
    let mix = *ports.mix * 0.01;

    if !self.is_active {
      self
        .grain_delay
        .initialize_params(pitch, filter, feedback, mix);
      self.is_active = true;
    }

    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());
    for (input, (out_left, out_right)) in ports.input.iter().zip(output_channels) {
      let (grain_delay_left, grain_delay_right) = self.grain_delay.process(
        *input, spray, freq, pitch, drift, reverse, time, feedback, filter, spread, mix,
      );
      *out_left = grain_delay_left;
      *out_right = grain_delay_right;
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmGrainDelay);
