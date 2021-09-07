extern crate lv2;
use lv2::prelude::*;
mod dbtoa;
use dbtoa::Dbtoa;
mod octaver;
use octaver::Octaver;

#[derive(PortCollection)]
struct Ports {
    gain: InputPort<Control>,
    threshold: InputPort<Control>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Octaver")]
struct DmOctaver {
    octaver: Octaver,
}

impl Plugin for DmOctaver {
    // Tell the framework which ports this plugin has.
    type Ports = Ports;

    // We don't need any special host features; We can leave them out.
    type InitFeatures = ();
    type AudioFeatures = ();

    // Create a new instance of the plugin; Trivial in this case.
    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self {
            octaver: Octaver::new(48000.),
        })
    }

    // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
    // iterates over.
    fn run(&mut self, ports: &mut Ports, _features: &mut ()) {
        let gain = Dbtoa::run(*(ports.gain));
        let threshold = Dbtoa::run(*(ports.threshold));

        for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            *out_frame = self.octaver.run(*in_frame, threshold, gain);
        }
    }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmOctaver);
