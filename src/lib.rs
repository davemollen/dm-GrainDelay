extern crate lv2;
use lv2::prelude::*;
mod octaver;
use octaver::Octaver;
mod dbtoa;
use dbtoa::Dbtoa;

#[derive(PortCollection)]
struct Ports {
    threshold: InputPort<Control>,
    gain: InputPort<Control>,
    mix: InputPort<Control>,
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
            octaver: Octaver::new(_plugin_info.sample_rate()),
        })
    }

    // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
    // iterates over.
    fn run(&mut self, ports: &mut Ports, _features: &mut ()) {
        let threshold = Dbtoa::run(*ports.threshold);
        let gain = Dbtoa::run(*ports.gain);
        let mix = *ports.mix * 0.01;
        for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            *out_frame = self.octaver.run(*in_frame, threshold, gain, mix);
        }
    }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmOctaver);
