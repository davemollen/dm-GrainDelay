extern crate lv2;
extern crate rand;
use lv2::prelude::*;
mod graindelay;
use graindelay::GrainDelay;

#[derive(PortCollection)]
struct Ports {
    spray: InputPort<Control>,
    pitch: InputPort<Control>,
    frequency: InputPort<Control>,
    rand_pitch: InputPort<Control>,
    delay_time: InputPort<Control>,
    feedback: InputPort<Control>,
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
            grain_delay: GrainDelay::new(_plugin_info.sample_rate()),
        })
    }

    // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
    // iterates over.
    fn run(&mut self, ports: &mut Ports, _features: &mut ()) {
        let pitch = *ports.pitch;
        let spray = *ports.spray;
        let frequency = *ports.frequency;
        let rand_pitch = *ports.rand_pitch;
        let delay_time = *ports.delay_time;
        let feedback = *ports.feedback * 0.01;
        let mix = *ports.mix * 0.01;

        for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            *out_frame = self.grain_delay.run(
                *in_frame, pitch, spray, frequency, rand_pitch, delay_time, feedback, mix,
            );
        }
    }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmGrainDelay);
